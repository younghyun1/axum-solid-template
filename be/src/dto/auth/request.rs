use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SignupRole {
    #[default]
    User,
    ServiceProvider,
}

#[derive(Debug, Deserialize, ToSchema, Zeroize, ZeroizeOnDrop)]
pub struct SignupRequest {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    #[serde(default)]
    #[zeroize(skip)]
    pub user_role: SignupRole,
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyEmailChallengeRequest {
    pub email_validation_token_id: Uuid,
    pub email_verification_challenge_id: Uuid,
    pub email_verification_pow_nonce: String,
    pub email_verification_answer: String,
    pub email_verification_honeypot: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEmailVerificationQuestionRequest {
    pub email_verification_question_prompt: String,
    pub email_verification_question_answers: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEmailVerificationQuestionAnswerRequest {
    pub email_verification_question_answer_text: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{SignupRequest, SignupRole};

    /// Perform the `signup_role_defaults_to_user` operation as implemented by this function.
    ///
    /// # Returns
    /// No value is returned (`()`).
    #[test]
    fn signup_role_defaults_to_user() {
        let decoded = serde_json::from_value::<SignupRequest>(json!({
            "user_name": "example",
            "user_email": "user@example.com",
            "user_password": "Password1!",
            "user_country": 840,
            "user_language": 840,
            "user_subdivision": null
        }));

        assert!(
            decoded.is_ok(),
            "failed to decode signup request: {decoded:?}"
        );
        let request = match decoded {
            Ok(request) => request,
            Err(_) => return,
        };

        assert_eq!(request.user_role, SignupRole::User);
    }

    /// Perform the `signup_role_accepts_service_provider` operation as implemented by this function.
    ///
    /// # Returns
    /// No value is returned (`()`).
    #[test]
    fn signup_role_accepts_service_provider() {
        let decoded = serde_json::from_value::<SignupRequest>(json!({
            "user_name": "provider",
            "user_email": "provider@example.com",
            "user_password": "Password1!",
            "user_role": "service_provider",
            "user_country": 840,
            "user_language": 840,
            "user_subdivision": null
        }));

        assert!(
            decoded.is_ok(),
            "failed to decode signup request: {decoded:?}"
        );
        let request = match decoded {
            Ok(request) => request,
            Err(_) => return,
        };

        assert_eq!(request.user_role, SignupRole::ServiceProvider);
    }
}
