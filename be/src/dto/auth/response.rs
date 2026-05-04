use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::auth::{
    email_verification_challenge::EmailVerificationQuestion, jwt::AccessTokenClaims, user::UserInfo,
};

#[derive(Debug, Serialize, ToSchema)]
pub struct SignupResponse {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub verify_by: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_at: DateTime<Utc>,
    pub claims: AccessTokenClaims,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MeResponse {
    pub user_info: UserInfo,
    pub claims: AccessTokenClaims,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub message: &'static str,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckIfUserExistsResponse {
    pub email_exists: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResetPasswordRequestResponse {
    pub user_email: String,
    pub verify_by: DateTime<Utc>,
    pub delivery_queued: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResetPasswordResponse {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyEmailResponse {
    pub user_id: Uuid,
    pub user_email: String,
    pub verified_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailVerificationChallengeResponse {
    pub email_verification_challenge_id: Uuid,
    pub email_verification_question_id: Uuid,
    pub email_verification_question_prompt: String,
    pub email_verification_pow_salt: String,
    pub email_verification_pow_difficulty_bits: i32,
    pub email_verification_pow_algorithm: String,
    pub email_verification_minimum_elapsed_ms: i32,
    pub email_verification_challenge_expires_at: DateTime<Utc>,
    pub email_verification_questionnaire_revision: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailVerificationQuestionnaireResponse {
    pub email_verification_questionnaire_revision: i64,
    pub email_verification_questions: Vec<EmailVerificationQuestion>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DatabaseResetResponse {
    pub reverted_migration_count: usize,
    pub applied_migration_count: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PublicUserInfoResponse {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_created_at: DateTime<Utc>,
    pub user_country: i32,
}
