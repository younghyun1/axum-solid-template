use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controller::v1::healthcheck::healthcheck,
        crate::controller::v1::auth::signup,
        crate::controller::v1::auth::login,
        crate::controller::v1::auth::me,
        crate::controller::v1::auth::logout,
        crate::controller::v1::auth::check_if_user_exists,
        crate::controller::v1::auth::reset_password_request,
        crate::controller::v1::auth::reset_password,
        crate::controller::v1::auth::verify_user_email,
        crate::controller::v1::auth::get_user_info,
        crate::controller::v1::reference_data::countries,
        crate::controller::v1::reference_data::languages,
        crate::controller::v1::reference_data::country_subdivisions
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
        crate::dto::healthcheck::HealthcheckResponse,
        crate::dto::reference_data::ReferenceCountryResponse,
        crate::dto::reference_data::ReferenceLanguageResponse,
        crate::dto::reference_data::ReferenceSubdivisionResponse
    )),
    tags(
        (name = "server", description = "Server health and runtime endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "user", description = "User endpoints"),
        (name = "reference", description = "Cached reference data endpoints")
    )
)]
pub struct ApiDoc;
