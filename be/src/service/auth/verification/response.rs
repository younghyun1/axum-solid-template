use crate::{
    domain::auth::email_verification_challenge::{
        EmailVerificationChallengeRow, EmailVerificationQuestion,
        EmailVerificationQuestionnaireSnapshot,
    },
    dto::auth::response::{
        EmailVerificationChallengeResponse, EmailVerificationQuestionnaireResponse,
    },
};

pub fn challenge_response(
    challenge: &EmailVerificationChallengeRow,
    question: &EmailVerificationQuestion,
    revision: i64,
) -> EmailVerificationChallengeResponse {
    EmailVerificationChallengeResponse {
        email_verification_challenge_id: challenge.email_verification_challenge_id,
        email_verification_question_id: question.email_verification_question_id,
        email_verification_question_prompt: question.email_verification_question_prompt.clone(),
        email_verification_pow_salt: challenge.email_verification_challenge_pow_salt.clone(),
        email_verification_pow_difficulty_bits: challenge
            .email_verification_challenge_pow_difficulty_bits,
        email_verification_pow_algorithm: challenge
            .email_verification_challenge_pow_algorithm
            .clone(),
        email_verification_minimum_elapsed_ms: challenge
            .email_verification_challenge_minimum_elapsed_ms,
        email_verification_challenge_expires_at: challenge.email_verification_challenge_expires_at,
        email_verification_questionnaire_revision: revision,
    }
}

pub fn questionnaire_response(
    snapshot: EmailVerificationQuestionnaireSnapshot,
) -> EmailVerificationQuestionnaireResponse {
    EmailVerificationQuestionnaireResponse {
        email_verification_questionnaire_revision: snapshot
            .email_verification_questionnaire_revision,
        email_verification_questions: snapshot.email_verification_questions,
    }
}
