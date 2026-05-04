#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplaceConfig {
    pub search_index_path: String,
    pub cache_dir: String,
    pub cache_capacity: u64,
    pub cache_ttl_seconds: u64,
}

impl MarketplaceConfig {
    pub fn new(
        search_index_path: String,
        cache_dir: String,
        cache_capacity: u64,
        cache_ttl_seconds: u64,
    ) -> Self {
        Self {
            search_index_path,
            cache_dir,
            cache_capacity,
            cache_ttl_seconds,
        }
    }
}
