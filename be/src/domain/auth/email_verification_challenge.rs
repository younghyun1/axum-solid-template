use chrono::{DateTime, Utc};
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Integer, Nullable, Text, Timestamptz, Uuid as SqlUuid};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

pub const QUESTION_STATUS_ACTIVE: &str = "active";
pub const QUESTION_STATUS_DELETED: &str = "deleted";
pub const ANSWER_STATUS_ACTIVE: &str = "active";
pub const ANSWER_STATUS_DELETED: &str = "deleted";
pub const CHALLENGE_STATUS_ISSUED: &str = "issued";
pub const CHALLENGE_STATUS_SOLVED: &str = "solved";
pub const CHALLENGE_STATUS_FAILED: &str = "failed";
pub const CHALLENGE_STATUS_EXPIRED: &str = "expired";
pub const CHALLENGE_STATUS_SUPERSEDED: &str = "superseded";
pub const POW_ALGORITHM_SHA256_V1: &str = "sha256-v1";

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EmailVerificationQuestion {
    pub email_verification_question_id: Uuid,
    pub email_verification_question_prompt: String,
    pub email_verification_question_answers: Vec<EmailVerificationQuestionAnswer>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EmailVerificationQuestionAnswer {
    pub email_verification_question_answer_id: Uuid,
    pub email_verification_question_answer_text: String,
    pub email_verification_question_answer_normalized: String,
}

#[derive(Debug, Clone)]
pub struct EmailVerificationQuestionnaireSnapshot {
    pub email_verification_questionnaire_revision: i64,
    pub email_verification_questions: Vec<EmailVerificationQuestion>,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct EmailVerificationQuestionnaireRevisionRow {
    #[diesel(sql_type = BigInt)]
    pub email_verification_questionnaire_revision: i64,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct EmailVerificationQuestionAnswerRow {
    #[diesel(sql_type = SqlUuid)]
    pub email_verification_question_id: Uuid,
    #[diesel(sql_type = Text)]
    pub email_verification_question_prompt: String,
    #[diesel(sql_type = Nullable<SqlUuid>)]
    pub email_verification_question_answer_id: Option<Uuid>,
    #[diesel(sql_type = Nullable<Text>)]
    pub email_verification_question_answer_text: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub email_verification_question_answer_normalized: Option<String>,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct CreatedEmailVerificationQuestionRow {
    #[diesel(sql_type = SqlUuid)]
    pub email_verification_question_id: Uuid,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct CreatedEmailVerificationQuestionAnswerRow {
    #[diesel(sql_type = SqlUuid)]
    pub email_verification_question_answer_id: Uuid,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct EmailVerificationChallengeRow {
    #[diesel(sql_type = SqlUuid)]
    pub email_verification_challenge_id: Uuid,
    #[diesel(sql_type = SqlUuid)]
    pub email_verification_token_id: Uuid,
    #[diesel(sql_type = SqlUuid)]
    pub email_verification_question_id: Uuid,
    #[diesel(sql_type = Text)]
    pub email_verification_challenge_pow_salt: String,
    #[diesel(sql_type = Integer)]
    pub email_verification_challenge_pow_difficulty_bits: i32,
    #[diesel(sql_type = Text)]
    pub email_verification_challenge_pow_algorithm: String,
    #[diesel(sql_type = Integer)]
    pub email_verification_challenge_minimum_elapsed_ms: i32,
    #[diesel(sql_type = Text)]
    pub email_verification_challenge_status: String,
    #[diesel(sql_type = Timestamptz)]
    pub email_verification_challenge_issued_at: DateTime<Utc>,
    #[diesel(sql_type = Timestamptz)]
    pub email_verification_challenge_expires_at: DateTime<Utc>,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    pub email_verification_challenge_solved_at: Option<DateTime<Utc>>,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    pub email_verification_challenge_failed_at: Option<DateTime<Utc>>,
    #[diesel(sql_type = Integer)]
    pub email_verification_challenge_attempt_count: i32,
}

#[derive(Debug, Clone)]
pub struct NewEmailVerificationChallenge {
    pub email_verification_challenge_id: Uuid,
    pub email_verification_token_id: Uuid,
    pub email_verification_question_id: Uuid,
    pub email_verification_challenge_pow_salt: String,
    pub email_verification_challenge_pow_difficulty_bits: i32,
    pub email_verification_challenge_pow_algorithm: String,
    pub email_verification_challenge_minimum_elapsed_ms: i32,
    pub email_verification_challenge_issued_at: DateTime<Utc>,
    pub email_verification_challenge_expires_at: DateTime<Utc>,
    pub email_verification_challenge_client_ip: Option<String>,
    pub email_verification_challenge_user_agent: Option<String>,
}
