use crate::{
    domain::marketplace::search::{MarketplaceSearchDocument, MarketplaceSearchResultKind},
    init::{
        db_pool::{DbPool, get_conn},
        state::search::marketplace::error::{MarketplaceSearchError, query_error},
    },
    repository::marketplace::postgres::{admin_repository, provider_repository},
};

pub async fn load_documents(
    db_pool: &DbPool,
) -> Result<Vec<MarketplaceSearchDocument>, MarketplaceSearchError> {
    let mut conn = match get_conn(db_pool).await {
        Ok(conn) => conn,
        Err(error) => return Err(MarketplaceSearchError::DbPool { error }),
    };

    let providers =
        match provider_repository::list_public_providers(&mut conn, None, None, 10_000).await {
            Ok(providers) => providers,
            Err(error) => return Err(query_error(error)),
        };
    let provider_posts =
        match provider_repository::list_public_provider_blog_posts_for_search(&mut conn).await {
            Ok(posts) => posts,
            Err(error) => return Err(query_error(error)),
        };
    let central_posts =
        match admin_repository::list_public_central_blog_posts_for_search(&mut conn).await {
            Ok(posts) => posts,
            Err(error) => return Err(query_error(error)),
        };

    let mut documents =
        Vec::with_capacity(providers.len() + provider_posts.len() + central_posts.len());
    for provider in providers {
        let subtitle = match provider.provider_profile_headline {
            Some(headline) => headline,
            None => String::from("Published provider profile"),
        };
        let body = if let Some(bio) = provider.provider_profile_bio {
            bio
        } else {
            String::with_capacity(0)
        };
        documents.push(MarketplaceSearchDocument {
            kind: MarketplaceSearchResultKind::Provider,
            title: provider.provider_profile_display_name,
            subtitle,
            body,
            slug: provider.provider_profile_slug.clone(),
            url_path: format!("/providers/{}", provider.provider_profile_slug),
            updated_at: provider.provider_profile_updated_at,
        });
    }
    for (post, provider) in provider_posts {
        let subtitle = match post.provider_blog_post_excerpt {
            Some(excerpt) => excerpt,
            None => provider.provider_profile_display_name.clone(),
        };
        documents.push(MarketplaceSearchDocument {
            kind: MarketplaceSearchResultKind::ProviderBlog,
            title: post.provider_blog_post_title,
            subtitle,
            body: post.provider_blog_post_body,
            slug: post.provider_blog_post_slug.clone(),
            url_path: format!(
                "/providers/{}/blog/{}",
                provider.provider_profile_slug, post.provider_blog_post_slug
            ),
            updated_at: post.provider_blog_post_updated_at,
        });
    }
    for post in central_posts {
        let subtitle = match post.central_blog_post_excerpt {
            Some(excerpt) => excerpt,
            None => String::from("Marketplace blog"),
        };
        documents.push(MarketplaceSearchDocument {
            kind: MarketplaceSearchResultKind::CentralBlog,
            title: post.central_blog_post_title,
            subtitle,
            body: post.central_blog_post_body,
            slug: post.central_blog_post_slug.clone(),
            url_path: format!("/blog/{}", post.central_blog_post_slug),
            updated_at: post.central_blog_post_updated_at,
        });
    }

    Ok(documents)
}
