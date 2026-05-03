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

/// Normalize answer submissions and return deduplicated canonical answer pairs.
///
/// # Arguments
/// * `answers` - Raw answer strings from the caller.
///
/// # Returns
/// A vector of unique `NormalizedAnswer` entries, with display text and normalized variants.
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

/// Normalize an answer for matching by lowercasing and collapsing whitespace.
///
/// # Arguments
/// * `answer` - Raw answer text.
///
/// # Returns
/// A normalized string used for case-insensitive comparison.
pub fn normalize_answer(answer: &str) -> String {
    answer
        .split_whitespace()
        .map(str::to_ascii_lowercase)
        .collect::<Vec<String>>()
        .join(" ")
}

/// Resolve a question in a snapshot by identifier.
///
/// # Arguments
/// * `snapshot` - Snapshot containing available questions.
/// * `question_id` - Question UUID to locate.
///
/// # Returns
/// The matching `EmailVerificationQuestion`, or `None` when absent.
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

    /// Test that normalization ignores case when producing equivalent values.
    #[test]
    fn answer_normalization_is_case_insensitive() {
        assert_eq!(normalize_answer(" Blue "), "blue");
        assert_eq!(normalize_answer("bLuE"), "blue");
    }

    /// Test that normalization collapses repeated whitespace and tabs.
    #[test]
    fn answer_normalization_collapses_whitespace() {
        assert_eq!(
            normalize_answer("  secure   local\taccount "),
            "secure local account"
        );
    }
}
