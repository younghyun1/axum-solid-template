mod claude;
mod generation;
mod mistral;

pub use claude::{
    ANTHROPIC_API_BASE_URL, ANTHROPIC_API_KEY_HEADER_NAME, ANTHROPIC_API_VERSION,
    ANTHROPIC_BETA_HEADER_NAME, ANTHROPIC_MESSAGES_ENDPOINT_PATH, ANTHROPIC_VERSION_HEADER_NAME,
    ClaudeConfig, ClaudeGenerationConfig,
};
pub use generation::ChatGenerationConfig;
pub use mistral::{
    MISTRAL_API_BASE_URL, MISTRAL_AUTHORIZATION_HEADER_NAME,
    MISTRAL_CHAT_COMPLETIONS_ENDPOINT_PATH, MISTRAL_EMBEDDINGS_ENDPOINT_PATH, MistralConfig,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ChatbotConfig {
    pub chatbot_provider: Option<ChatbotProvider>,
    pub chatbot_rag_storage_provider: Option<RagStorageProvider>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatbotProvider {
    Claude(ClaudeConfig),
    Mistral(MistralConfig),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RagStorageProvider {
    InRam,
    PgVector,
}
