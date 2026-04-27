use super::{claude::ClaudeConfig, mistral::MistralConfig};

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
