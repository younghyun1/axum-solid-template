use axum::http::{HeaderMap, header};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum ContentCodingPreference {
    Zstd,
    Gzip,
    Identity,
}

fn parse_quality(raw: &str) -> f32 {
    match raw.trim().parse::<f32>() {
        Ok(value) if (0.0..=1.0).contains(&value) => value,
        Ok(_) => 0.0,
        Err(_) => 0.0,
    }
}

fn set_max_quality(slot: &mut Option<f32>, quality: f32) {
    match *slot {
        Some(current) if current >= quality => {}
        _ => *slot = Some(quality),
    }
}

fn q_value_from_parameter(parameter: &str) -> Option<f32> {
    let mut key_value = parameter.trim().splitn(2, '=');
    let key = match key_value.next() {
        Some(value) => value.trim(),
        None => return None,
    };

    match key.eq_ignore_ascii_case("q") {
        true => match key_value.next() {
            Some(raw_quality) => Some(parse_quality(raw_quality)),
            None => Some(0.0),
        },
        false => None,
    }
}

#[allow(clippy::manual_unwrap_or, clippy::single_match)]
pub(super) fn select_static_encoding(headers: &HeaderMap) -> ContentCodingPreference {
    let accept_encoding = match headers.get(header::ACCEPT_ENCODING) {
        Some(value) => match value.to_str() {
            Ok(parsed) => parsed,
            Err(error) => {
                tracing::warn!(
                    error = %error,
                    "Invalid Accept-Encoding header; serving identity asset"
                );
                return ContentCodingPreference::Identity;
            }
        },
        None => return ContentCodingPreference::Identity,
    };

    let mut zstd_q: Option<f32> = None;
    let mut gzip_q: Option<f32> = None;
    let mut identity_q: Option<f32> = None;
    let mut wildcard_q: Option<f32> = None;

    for encoding_entry in accept_encoding.split(',') {
        let mut parts = encoding_entry.trim().split(';');
        let encoding_name = match parts.next() {
            Some(value) => value.trim().to_ascii_lowercase(),
            None => continue,
        };
        if encoding_name.is_empty() {
            continue;
        }

        let mut quality = 1.0_f32;
        for parameter in parts {
            match q_value_from_parameter(parameter) {
                Some(parsed_quality) => quality = parsed_quality,
                None => {}
            }
        }

        match encoding_name.as_str() {
            "zstd" => set_max_quality(&mut zstd_q, quality),
            "gzip" | "x-gzip" => set_max_quality(&mut gzip_q, quality),
            "identity" => set_max_quality(&mut identity_q, quality),
            "*" => set_max_quality(&mut wildcard_q, quality),
            _ => {}
        }
    }

    let wildcard_default = match wildcard_q {
        Some(value) => value,
        None => 0.0,
    };
    let zstd_effective = match zstd_q {
        Some(value) => value,
        None => wildcard_default,
    };
    let gzip_effective = match gzip_q {
        Some(value) => value,
        None => wildcard_default,
    };
    let identity_effective = match identity_q {
        Some(value) => value,
        None => match wildcard_q {
            Some(0.0) => 0.0,
            _ => 1.0,
        },
    };

    if zstd_effective > 0.0
        && zstd_effective >= gzip_effective
        && zstd_effective >= identity_effective
    {
        return ContentCodingPreference::Zstd;
    }

    if gzip_effective > 0.0 && gzip_effective >= identity_effective {
        return ContentCodingPreference::Gzip;
    }

    ContentCodingPreference::Identity
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::*;

    fn accept_encoding_headers(value: &'static str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT_ENCODING, HeaderValue::from_static(value));
        headers
    }

    #[test]
    fn select_static_encoding_prefers_zstd_when_quality_is_highest() {
        let headers = accept_encoding_headers("gzip;q=0.8, zstd;q=1.0");

        assert_eq!(
            select_static_encoding(&headers),
            ContentCodingPreference::Zstd
        );
    }

    #[test]
    fn select_static_encoding_respects_disabled_zstd() {
        let headers = accept_encoding_headers("zstd;q=0, gzip;q=1.0");

        assert_eq!(
            select_static_encoding(&headers),
            ContentCodingPreference::Gzip
        );
    }

    #[test]
    fn select_static_encoding_defaults_to_identity_without_header() {
        let headers = HeaderMap::new();

        assert_eq!(
            select_static_encoding(&headers),
            ContentCodingPreference::Identity
        );
    }
}
