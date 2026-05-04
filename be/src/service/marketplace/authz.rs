use crate::{
    domain::auth::jwt::AccessTokenClaims,
    dto::api_response::ApiResult,
    error::{api_error::ApiError, code_error::CodeError},
};

pub fn require_provider(claims: &AccessTokenClaims) -> ApiResult<()> {
    if claims.is_service_provider() || claims.is_admin() {
        return Ok(());
    }
    Err(ApiError::new(CodeError::SERVICE_PROVIDER_REQUIRED))
}

pub fn require_moderator(claims: &AccessTokenClaims) -> ApiResult<()> {
    if claims.is_moderator() || claims.is_admin() {
        return Ok(());
    }
    Err(ApiError::new(CodeError::MODERATOR_REQUIRED))
}
