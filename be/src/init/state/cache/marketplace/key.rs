use crate::domain::marketplace::enums::BannerPlacement;

pub fn provider_directory(
    query: Option<&str>,
    subdivision_id: Option<i32>,
    subdivision_code: Option<&str>,
    limit: i64,
) -> String {
    format!(
        "public:providers:q={}:sid={}:scode={}:limit={}",
        normalize(query),
        subdivision_id
            .map(|value| value.to_string())
            .unwrap_or_default(),
        normalize(subdivision_code),
        limit
    )
}

pub fn provider_detail(slug: &str) -> String {
    format!("public:provider:{slug}")
}

pub fn provider_blog_post(provider_slug: &str, post_slug: &str) -> String {
    format!("public:provider:{provider_slug}:blog:{post_slug}")
}

pub fn active_banners(placement: BannerPlacement) -> String {
    format!("public:banners:{}", placement_key(placement))
}

fn normalize(value: Option<&str>) -> String {
    match value {
        Some(value) => value.trim().to_ascii_lowercase(),
        None => String::new(),
    }
}

fn placement_key(placement: BannerPlacement) -> &'static str {
    match placement {
        BannerPlacement::HomepageTop => "homepage_top",
        BannerPlacement::DirectorySidebar => "directory_sidebar",
        BannerPlacement::ProviderProfile => "provider_profile",
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::marketplace::enums::BannerPlacement,
        init::state::cache::marketplace::key::{
            active_banners, provider_blog_post, provider_detail, provider_directory,
        },
    };

    #[test]
    fn provider_directory_key_normalizes_query_values() {
        let key = provider_directory(Some("  Roofing "), Some(2826), Some(" GB-LND "), 24);

        assert_eq!(
            key,
            "public:providers:q=roofing:sid=2826:scode=gb-lnd:limit=24"
        );
    }

    #[test]
    fn provider_detail_key_uses_slug() {
        let key = provider_detail("north-shop");

        assert_eq!(key, "public:provider:north-shop");
    }

    #[test]
    fn provider_blog_key_uses_provider_and_post_slug() {
        let key = provider_blog_post("north-shop", "spring-launch");

        assert_eq!(key, "public:provider:north-shop:blog:spring-launch");
    }

    #[test]
    fn banner_key_uses_wire_placement() {
        let key = active_banners(BannerPlacement::DirectorySidebar);

        assert_eq!(key, "public:banners:directory_sidebar");
    }
}
