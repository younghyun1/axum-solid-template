use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookieSameSitePolicy {
    Lax,
}

impl CookieSameSitePolicy {
    pub fn as_set_cookie_value(self) -> &'static str {
        match self {
            Self::Lax => "Lax",
        }
    }
}

impl fmt::Display for CookieSameSitePolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_set_cookie_value())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CookieConfig {
    pub secure: bool,
    pub same_site: CookieSameSitePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allow_credentials: bool,
}

impl CorsConfig {
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        let normalized = match normalize_origin(origin) {
            Some(normalized) => normalized,
            None => return false,
        };

        self.allowed_origins
            .iter()
            .any(|allowed| allowed == &normalized)
    }
}

pub fn normalize_origin(value: &str) -> Option<String> {
    let trimmed = value.trim().trim_end_matches('/');
    if trimmed.is_empty()
        || trimmed.contains('*')
        || trimmed.contains(',')
        || trimmed.contains(';')
        || trimmed.chars().any(char::is_control)
    {
        return None;
    }

    let (scheme, authority) = match trimmed.split_once("://") {
        Some(parts) => parts,
        None => return None,
    };
    let normalized_scheme = scheme.to_ascii_lowercase();
    if normalized_scheme != "http" && normalized_scheme != "https" {
        return None;
    }
    if authority.is_empty()
        || authority.contains('/')
        || authority.contains('?')
        || authority.contains('#')
        || authority.chars().any(char::is_whitespace)
    {
        return None;
    }

    Some(format!(
        "{}://{}",
        normalized_scheme,
        authority.to_ascii_lowercase()
    ))
}

#[cfg(test)]
mod tests {
    use super::normalize_origin;

    #[test]
    fn normalize_origin_accepts_plain_origins() {
        assert_eq!(
            normalize_origin("HTTPS://Example.COM/"),
            Some("https://example.com".to_string())
        );
    }

    #[test]
    fn normalize_origin_rejects_wildcards_and_paths() {
        assert_eq!(normalize_origin("*"), None);
        assert_eq!(normalize_origin("https://example.com/path"), None);
    }
}
