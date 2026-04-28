use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controller::healthcheck::healthcheck,
        crate::controller::auth::signup,
        crate::controller::auth::login,
        crate::controller::auth::me,
        crate::controller::auth::logout,
        crate::controller::auth::check_if_user_exists,
        crate::controller::auth::reset_password_request,
        crate::controller::auth::reset_password,
        crate::controller::auth::verify_user_email,
        crate::controller::auth::get_user_info
    ),
    components(schemas(
        crate::domain::auth::jwt::AccessTokenClaims,
        crate::domain::auth::jwt::JwtTokenType,
        crate::domain::auth::role::RoleType,
        crate::domain::auth::user::UserInfo,
        crate::dto::api_response::ApiErrorBody,
        crate::dto::api_response::ApiErrorLevel,
        crate::dto::api_response::ApiMeta,
        crate::dto::auth::request::SignupRequest,
        crate::dto::auth::request::LoginRequest,
        crate::dto::auth::request::CheckIfUserExistsRequest,
        crate::dto::auth::request::ResetPasswordRequest,
        crate::dto::auth::request::ResetPasswordProcessRequest,
        crate::dto::auth::request::EmailValidationToken,
        crate::dto::auth::response::SignupResponse,
        crate::dto::auth::response::LoginResponse,
        crate::dto::auth::response::MeResponse,
        crate::dto::auth::response::LogoutResponse,
        crate::dto::auth::response::CheckIfUserExistsResponse,
        crate::dto::auth::response::ResetPasswordRequestResponse,
        crate::dto::auth::response::ResetPasswordResponse,
        crate::dto::auth::response::VerifyEmailResponse,
        crate::dto::auth::response::PublicUserInfoResponse,
        crate::dto::healthcheck::HealthcheckResponse
    )),
    tags(
        (name = "server", description = "Server health and runtime endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "user", description = "User endpoints")
    )
)]
pub struct ApiDoc;
