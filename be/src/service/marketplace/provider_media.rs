use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    domain::{
        auth::jwt::AccessTokenClaims,
        marketplace::{
            enums::{ImageType, ImageUploadStatus},
            media::{Image, NewImage},
        },
    },
    dto::{
        api_response::ApiResult,
        marketplace::{
            request::{CompleteImageUploadRequest, CreateImageRequest},
            response::ImageResponse,
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::marketplace::postgres::{media_repository, provider_repository},
    service::{
        auth::datasource::postgres_conn,
        marketplace::{authz, cache, indexing, validation},
    },
};

pub async fn create_provider_image(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    request: CreateImageRequest,
) -> ApiResult<ImageResponse> {
    match authz::require_provider(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    match validation::validate_image_metadata(
        &request.bucket,
        &request.object_key,
        &request.mime_type,
        request.byte_size,
        request.width,
        request.height,
    ) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let profile =
        match provider_repository::find_provider_profile_by_user(&mut conn, claims.user_id).await {
            Ok(Some(profile)) => profile,
            Ok(None) => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };

    let provider_blog_post_id = match request.image_type {
        ImageType::ProviderBlog => {
            match provider_blog_owner(
                &mut conn,
                profile.provider_profile_id,
                request.provider_blog_post_id,
            )
            .await
            {
                Ok(owner) => owner,
                Err(error) => return Err(error),
            }
        }
        ImageType::ProviderProfile => None,
        ImageType::UserProfile | ImageType::CentralBlog | ImageType::AdvertisementBanner => {
            return Err(ApiError::public(
                CodeError::VALIDATION_FAILED,
                "provider image endpoint only accepts provider profile or provider blog images",
            ));
        }
    };

    let provider_profile_id = match request.image_type {
        ImageType::ProviderProfile => Some(profile.provider_profile_id),
        ImageType::ProviderBlog
        | ImageType::UserProfile
        | ImageType::CentralBlog
        | ImageType::AdvertisementBanner => None,
    };

    let new_image = NewImage {
        image_type: request.image_type,
        image_upload_status: ImageUploadStatus::Pending,
        image_visibility: request.visibility,
        image_bucket: request.bucket,
        image_object_key: request.object_key,
        image_public_url: request.public_url,
        image_mime_type: request.mime_type,
        image_byte_size: request.byte_size,
        image_width: request.width,
        image_height: request.height,
        image_checksum_sha256: request.checksum_sha256,
        user_id: None,
        provider_profile_id,
        provider_blog_post_id,
        central_blog_post_id: None,
        advertisement_banner_id: None,
    };

    match media_repository::insert_image(&mut conn, new_image).await {
        Ok(image) => {
            cache::clear_public_cache(&state, "provider_image_create").await;
            indexing::rebuild_search_index(&state, "provider_image_create").await;
            Ok(ImageResponse::from(image))
        }
        Err(error) => Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
    }
}

pub async fn complete_provider_image_upload(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    image_id: Uuid,
    request: CompleteImageUploadRequest,
) -> ApiResult<ImageResponse> {
    match authz::require_provider(&claims) {
        Ok(()) => {}
        Err(error) => return Err(error),
    }
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };
    let profile =
        match provider_repository::find_provider_profile_by_user(&mut conn, claims.user_id).await {
            Ok(Some(profile)) => profile,
            Ok(None) => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };
    let image = match media_repository::find_image_by_id(&mut conn, image_id).await {
        Ok(Some(image)) => image,
        Ok(None) => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };
    match image_belongs_to_provider(&mut conn, profile.provider_profile_id, &image).await {
        Ok(()) => {}
        Err(error) => return Err(error),
    }

    match media_repository::complete_image_upload(
        &mut conn,
        image_id,
        request.public_url,
        Utc::now(),
    )
    .await
    {
        Ok(image) => {
            cache::clear_public_cache(&state, "provider_image_complete").await;
            indexing::rebuild_search_index(&state, "provider_image_complete").await;
            Ok(ImageResponse::from(image))
        }
        Err(error) => Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error)),
    }
}

async fn provider_blog_owner(
    conn: &mut diesel_async::AsyncPgConnection,
    provider_profile_id: Uuid,
    blog_post_id: Option<Uuid>,
) -> ApiResult<Option<Uuid>> {
    let blog_post_id = match blog_post_id {
        Some(blog_post_id) => blog_post_id,
        None => {
            return Err(ApiError::public(
                CodeError::VALIDATION_FAILED,
                "provider_blog_post_id is required for provider blog images",
            ));
        }
    };

    match provider_owns_blog_post(conn, provider_profile_id, blog_post_id).await {
        Ok(()) => Ok(Some(blog_post_id)),
        Err(error) => Err(error),
    }
}

async fn provider_owns_blog_post(
    conn: &mut diesel_async::AsyncPgConnection,
    provider_profile_id: Uuid,
    blog_post_id: Uuid,
) -> ApiResult<()> {
    let posts = match provider_repository::list_provider_blog_posts(conn, provider_profile_id).await
    {
        Ok(posts) => posts,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    };
    if posts
        .iter()
        .any(|post| post.provider_blog_post_id == blog_post_id)
    {
        return Ok(());
    }
    Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND))
}

async fn image_belongs_to_provider(
    conn: &mut diesel_async::AsyncPgConnection,
    provider_profile_id: Uuid,
    image: &Image,
) -> ApiResult<()> {
    match image.image_type {
        ImageType::ProviderProfile => {
            if image.provider_profile_id == Some(provider_profile_id) {
                return Ok(());
            }
        }
        ImageType::ProviderBlog => {
            let blog_post_id = match image.provider_blog_post_id {
                Some(blog_post_id) => blog_post_id,
                None => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
            };
            return provider_owns_blog_post(conn, provider_profile_id, blog_post_id).await;
        }
        ImageType::UserProfile | ImageType::CentralBlog | ImageType::AdvertisementBanner => {}
    }
    Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND))
}
