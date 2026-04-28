pub fn validate_username(username: &str) -> bool {
    let trimmed = username.trim();

    if trimmed.len() < 3 || trimmed.len() > 32 {
        return false;
    }

    trimmed
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-')
}

pub fn validate_password_form(password: &str) -> bool {
    if password.len() < 12 || password.len() > 256 {
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

pub fn normalized_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::{validate_password_form, validate_username};

    #[test]
    fn validates_username_shape() {
        assert!(validate_username("valid_user-1"));
        assert!(!validate_username("no spaces"));
        assert!(!validate_username("x"));
    }

    #[test]
    fn validates_password_shape() {
        assert!(validate_password_form("LongPassword1!"));
        assert!(!validate_password_form("short1!A"));
        assert!(!validate_password_form("longpassword1!"));
    }
}
