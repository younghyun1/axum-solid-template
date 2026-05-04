use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceSearchResultKind {
    Provider,
    ProviderBlog,
    CentralBlog,
}

impl MarketplaceSearchResultKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Provider => "provider",
            Self::ProviderBlog => "provider_blog",
            Self::CentralBlog => "central_blog",
        }
    }

    pub fn from_wire_value(value: &str) -> Option<Self> {
        match value {
            "provider" => Some(Self::Provider),
            "provider_blog" => Some(Self::ProviderBlog),
            "central_blog" => Some(Self::CentralBlog),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarketplaceSearchDocument {
    pub kind: MarketplaceSearchResultKind,
    pub title: String,
    pub subtitle: String,
    pub body: String,
    pub slug: String,
    pub url_path: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MarketplaceSearchHit {
    pub kind: MarketplaceSearchResultKind,
    pub title: String,
    pub subtitle: String,
    pub slug: String,
    pub url_path: String,
    pub snippet: String,
    pub score: f32,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::MarketplaceSearchResultKind;

    #[test]
    fn maps_search_kind_to_wire_value() {
        assert_eq!(MarketplaceSearchResultKind::Provider.as_str(), "provider");
        assert_eq!(
            MarketplaceSearchResultKind::ProviderBlog.as_str(),
            "provider_blog"
        );
        assert_eq!(
            MarketplaceSearchResultKind::CentralBlog.as_str(),
            "central_blog"
        );
    }

    #[test]
    fn parses_search_kind_from_wire_value() {
        assert_eq!(
            MarketplaceSearchResultKind::from_wire_value("provider"),
            Some(MarketplaceSearchResultKind::Provider)
        );
        assert_eq!(
            MarketplaceSearchResultKind::from_wire_value("provider_blog"),
            Some(MarketplaceSearchResultKind::ProviderBlog)
        );
        assert_eq!(
            MarketplaceSearchResultKind::from_wire_value("unknown"),
            None
        );
    }
}
