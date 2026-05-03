use std::collections::HashSet;

use uuid::Uuid;

use crate::domain::auth::email_verification_challenge::{
    EmailVerificationQuestion, EmailVerificationQuestionnaireSnapshot,
};

#[derive(Debug)]
pub struct NormalizedAnswer {
    pub answer_text: String,
    pub answer_normalized: String,
}

pub fn normalize_answer_set(answers: Vec<String>) -> Vec<NormalizedAnswer> {
    let mut seen = HashSet::new();
    let mut normalized_answers = Vec::new();
    for answer in answers {
        let answer_text = answer.trim().to_string();
        let answer_normalized = normalize_answer(&answer_text);
        if answer_normalized.is_empty() {
            continue;
        }
        if !seen.insert(answer_normalized.clone()) {
            continue;
        }
        normalized_answers.push(NormalizedAnswer {
            answer_text,
            answer_normalized,
        });
    }
    normalized_answers
}

pub fn normalize_answer(answer: &str) -> String {
    answer
        .split_whitespace()
        .map(str::to_ascii_lowercase)
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn find_question(
    snapshot: &EmailVerificationQuestionnaireSnapshot,
    question_id: Uuid,
) -> Option<EmailVerificationQuestion> {
    snapshot
        .email_verification_questions
        .iter()
        .find(|question| question.email_verification_question_id == question_id)
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::normalize_answer;

    #[test]
    fn answer_normalization_is_case_insensitive() {
        assert_eq!(normalize_answer(" Blue "), "blue");
        assert_eq!(normalize_answer("bLuE"), "blue");
    }

    #[test]
    fn answer_normalization_collapses_whitespace() {
        assert_eq!(
            normalize_answer("  secure   local\taccount "),
            "secure local account"
        );
    }
}
