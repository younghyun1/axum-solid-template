use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Deserialize, ToSchema, Zeroize, ZeroizeOnDrop)]
pub struct SignupRequest {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_country: i32,
    pub user_language: i32,
    pub user_subdivision: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema, Zeroize, ZeroizeOnDrop)]
pub struct LoginRequest {
    pub user_email: String,
    pub user_password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CheckIfUserExistsRequest {
    pub user_email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ResetPasswordRequest {
    pub user_email: String,
}

#[derive(Debug, Deserialize, ToSchema, Zeroize, ZeroizeOnDrop)]
pub struct ResetPasswordProcessRequest {
    #[zeroize(skip)]
    pub password_reset_token: Uuid,
    pub new_password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EmailValidationToken {
    pub email_validation_token_id: Uuid,
}
