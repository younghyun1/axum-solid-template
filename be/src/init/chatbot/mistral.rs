use super::generation::ChatGenerationConfig;

pub const MISTRAL_API_BASE_URL: &str = "https://api.mistral.ai/v1";
pub const MISTRAL_AUTHORIZATION_HEADER_NAME: &str = "Authorization";
pub const MISTRAL_CHAT_COMPLETIONS_ENDPOINT_PATH: &str = "/chat/completions";
pub const MISTRAL_EMBEDDINGS_ENDPOINT_PATH: &str = "/embeddings";

#[derive(Debug, Clone, PartialEq)]
pub struct MistralConfig {
    pub mistral_api_key: String,
    pub mistral_base_url: String,
    pub mistral_chat_completions_endpoint_path: String,
    pub mistral_embeddings_endpoint_path: String,
    pub mistral_model: String,
    pub mistral_embedding_model: Option<String>,
    pub mistral_timeout_seconds: u64,
    pub mistral_max_retries: u8,
    pub mistral_generation_config: ChatGenerationConfig,
}

impl MistralConfig {
    pub fn public_api(api_key: String, model: String) -> Self {
        Self {
            mistral_api_key: api_key,
            mistral_base_url: MISTRAL_API_BASE_URL.to_owned(),
            mistral_chat_completions_endpoint_path: MISTRAL_CHAT_COMPLETIONS_ENDPOINT_PATH
                .to_owned(),
            mistral_embeddings_endpoint_path: MISTRAL_EMBEDDINGS_ENDPOINT_PATH.to_owned(),
            mistral_model: model,
            mistral_embedding_model: None,
            mistral_timeout_seconds: 60,
            mistral_max_retries: 2,
            mistral_generation_config: ChatGenerationConfig::disabled(),
        }
    }
}
