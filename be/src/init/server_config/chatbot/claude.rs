pub const ANTHROPIC_API_BASE_URL: &str = "https://api.anthropic.com/v1";
pub const ANTHROPIC_API_KEY_HEADER_NAME: &str = "x-api-key";
pub const ANTHROPIC_VERSION_HEADER_NAME: &str = "anthropic-version";
pub const ANTHROPIC_BETA_HEADER_NAME: &str = "anthropic-beta";
pub const ANTHROPIC_MESSAGES_ENDPOINT_PATH: &str = "/messages";
pub const ANTHROPIC_API_VERSION: &str = "2023-06-01";

#[derive(Debug, Clone, PartialEq)]
pub struct ClaudeConfig {
    pub anthropic_api_key: String,
    pub anthropic_base_url: String,
    pub anthropic_messages_endpoint_path: String,
    pub anthropic_version: String,
    pub anthropic_beta_features: Vec<String>,
    pub anthropic_model: String,
    pub anthropic_timeout_seconds: u64,
    pub anthropic_max_retries: u8,
    pub anthropic_generation_config: ClaudeGenerationConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaudeGenerationConfig {
    pub max_tokens: u32,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub stop_sequences: Vec<String>,
}

impl ClaudeGenerationConfig {
    pub fn with_max_tokens(max_tokens: u32) -> Self {
        Self {
            max_tokens,
            temperature: None,
            top_p: None,
            top_k: None,
            stop_sequences: Vec::new(),
        }
    }
}

impl ClaudeConfig {
    pub fn public_api(api_key: String, model: String, max_tokens: u32) -> Self {
        Self {
            anthropic_api_key: api_key,
            anthropic_base_url: ANTHROPIC_API_BASE_URL.to_owned(),
            anthropic_messages_endpoint_path: ANTHROPIC_MESSAGES_ENDPOINT_PATH.to_owned(),
            anthropic_version: ANTHROPIC_API_VERSION.to_owned(),
            anthropic_beta_features: Vec::new(),
            anthropic_model: model,
            anthropic_timeout_seconds: 60,
            anthropic_max_retries: 2,
            anthropic_generation_config: ClaudeGenerationConfig::with_max_tokens(max_tokens),
        }
    }
}
