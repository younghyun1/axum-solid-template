use std::sync::Arc;

use serde_json::json;

use crate::{
    domain::{
        auth::jwt::AccessTokenClaims,
        marketplace::{
            enums::{PaymentIntentStatus, PaymentTransactionKind, PaymentTransactionStatus},
            payments::{NewPaymentIntent, NewPaymentTransaction},
            provider::NewUserProfileExtension,
        },
    },
    dto::{
        api_response::ApiResult,
        marketplace::{
            request::{CreatePaymentIntentRequest, UpsertUserProfileRequest},
            response::{PaymentIntentListResponse, PaymentIntentResponse, UserProfileResponse},
        },
    },
    error::{api_error::ApiError, code_error::CodeError},
    init::state::server_state::ServerState,
    repository::marketplace::postgres::{payment_repository, provider_repository},
    service::{auth::datasource::postgres_conn, marketplace::validation},
};

pub async fn user_profile(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
) -> ApiResult<UserProfileResponse> {
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    match provider_repository::find_user_profile_extension(&mut conn, claims.user_id).await {
        Ok(Some(profile)) => Ok(UserProfileResponse::from(profile)),
        Ok(None) => Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
        Err(error) => Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    }
}

pub async fn upsert_user_profile(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    request: UpsertUserProfileRequest,
) -> ApiResult<UserProfileResponse> {
    let display_name = match validation::short_optional(request.display_name, "display_name") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let bio = match validation::long_optional(request.bio, "bio") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let phone = match validation::short_optional(request.phone, "phone") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let public_email = match validation::short_optional(request.public_email, "public_email") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    let existing =
        match provider_repository::find_user_profile_extension(&mut conn, claims.user_id).await {
            Ok(existing) => existing,
            Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
        };

    let profile = match existing {
        Some(_) => {
            match provider_repository::update_user_profile_extension(
                &mut conn,
                claims.user_id,
                display_name,
                bio,
                phone,
                public_email,
                chrono::Utc::now(),
            )
            .await
            {
                Ok(profile) => profile,
                Err(error) => return Err(ApiError::from_source(CodeError::DB_UPDATE_ERROR, error)),
            }
        }
        None => {
            let new_profile = NewUserProfileExtension {
                user_id: claims.user_id,
                user_profile_extension_display_name: display_name,
                user_profile_extension_bio: bio,
                user_profile_extension_phone: phone,
                user_profile_extension_public_email: public_email,
            };
            match provider_repository::insert_user_profile_extension(&mut conn, new_profile).await {
                Ok(profile) => profile,
                Err(error) => return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
            }
        }
    };

    Ok(UserProfileResponse::from(profile))
}

pub async fn create_payment_intent(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
    request: CreatePaymentIntentRequest,
) -> ApiResult<PaymentIntentResponse> {
    if request.amount_minor_units <= 0 {
        return Err(ApiError::public(
            CodeError::VALIDATION_FAILED,
            "amount_minor_units must be positive",
        ));
    }

    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    match provider_repository::find_provider_profile_by_id(&mut conn, request.provider_profile_id)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return Err(ApiError::new(CodeError::MARKETPLACE_NOT_FOUND)),
        Err(error) => return Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    }

    let new_intent = NewPaymentIntent {
        user_id: claims.user_id,
        provider_profile_id: request.provider_profile_id,
        payment_intent_amount_minor_units: request.amount_minor_units,
        payment_intent_currency: request.currency_code,
        payment_provider: request.payment_provider,
        payment_intent_status: PaymentIntentStatus::Created,
        payment_intent_metadata: json!({"origin": "website"}),
    };

    let intent = match payment_repository::insert_payment_intent(&mut conn, new_intent).await {
        Ok(intent) => intent,
        Err(error) => return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
    };

    let new_transaction = NewPaymentTransaction {
        payment_intent_id: intent.payment_intent_id,
        payment_transaction_kind: PaymentTransactionKind::Authorization,
        payment_transaction_status: PaymentTransactionStatus::Pending,
        payment_transaction_amount_minor_units: request.amount_minor_units,
        payment_transaction_currency: request.currency_code,
    };

    match payment_repository::insert_payment_transaction(&mut conn, new_transaction).await {
        Ok(_) => {}
        Err(error) => return Err(ApiError::from_source(CodeError::DB_INSERT_ERROR, error)),
    }

    Ok(PaymentIntentResponse::from(intent))
}

pub async fn list_payment_intents(
    state: Arc<ServerState>,
    claims: AccessTokenClaims,
) -> ApiResult<PaymentIntentListResponse> {
    let mut conn = match postgres_conn(&state).await {
        Ok(conn) => conn,
        Err(error) => return Err(error),
    };

    match payment_repository::list_user_payment_intents(&mut conn, claims.user_id).await {
        Ok(payment_intents) => Ok(PaymentIntentListResponse {
            payment_intents: payment_intents.into_iter().map(Into::into).collect(),
        }),
        Err(error) => Err(ApiError::from_source(CodeError::DB_QUERY_ERROR, error)),
    }
}
