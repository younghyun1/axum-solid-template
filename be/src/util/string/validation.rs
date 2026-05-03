use crate::domain::auth::value::{UserEmail, UserName};

const PASSWORD_MIN_CHARS: usize = 9;
const PASSWORD_MAX_CHARS: usize = 256;

/// Validate username format using the `UserName` domain validator.
///
/// # Arguments
/// * `username` - Raw user name to validate.
///
/// # Returns
/// `true` when the username passes domain validation, otherwise `false`.
pub fn validate_username(username: &str) -> bool {
    UserName::try_new(username.to_string()).is_ok()
}

/// Validate password against minimum/maximum length and character-class requirements.
///
/// # Arguments
/// * `password` - Candidate password text.
///
/// # Returns
/// `true` when all strength constraints are met.
pub fn validate_password_form(password: &str) -> bool {
    let character_count = password.chars().count();

    if !(PASSWORD_MIN_CHARS..=PASSWORD_MAX_CHARS).contains(&character_count) {
        return false;
    }

    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_digit = false;
    let mut has_symbol = false;

    for character in password.chars() {
        if character.is_ascii_lowercase() {
            has_lowercase = true;
            continue;
        }

        if character.is_ascii_uppercase() {
            has_uppercase = true;
            continue;
        }

        if character.is_ascii_digit() {
            has_digit = true;
            continue;
        }

        if character.is_ascii_punctuation() || character.is_ascii_whitespace() {
            has_symbol = true;
        }
    }

    has_lowercase && has_uppercase && has_digit && has_symbol
}

/// Normalize email input with domain-level parsing fallback.
///
/// # Arguments
/// * `email` - Raw email input.
///
/// # Returns
/// Canonicalized email when parseable by `UserEmail`, otherwise trimmed lowercase fallback.
pub fn normalized_email(email: &str) -> String {
    match UserEmail::try_new(email.to_string()) {
        Ok(user_email) => user_email.into_inner(),
        Err(_) => email.trim().to_ascii_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use super::{validate_password_form, validate_username};

    /// Validate username acceptance and rejection cases.
    #[test]
    fn validates_username_shape() {
        assert!(validate_username("valid_user-1"));
        assert!(!validate_username("no spaces"));
        assert!(!validate_username("x"));
    }

    /// Validate password shape acceptance and rejection cases.
    #[test]
    fn validates_password_shape() {
        assert!(validate_password_form("Aa123456!"));
        assert!(!validate_password_form("short1!A"));
        assert!(!validate_password_form("longpassword1!"));
    }
}
