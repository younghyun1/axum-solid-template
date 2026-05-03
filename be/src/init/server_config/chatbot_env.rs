use crate::init::server_config::{
    chatbot::{
        chatbot_config::{ChatbotConfig, ChatbotProvider, RagStorageProvider},
        claude::ClaudeConfig,
        mistral::MistralConfig,
    },
    server_config::ServerConfigError,
};

use super::parsers::{normalized_env_value, optional_env, required_env, required_int_env};

pub(super) fn chatbot_config_from_env() -> Result<ChatbotConfig, ServerConfigError> {
    let chatbot_provider = match chatbot_provider_from_env() {
        Ok(chatbot_provider) => chatbot_provider,
        Err(error) => return Err(error),
    };
    let chatbot_rag_storage_provider = match rag_storage_provider_from_env() {
        Ok(chatbot_rag_storage_provider) => chatbot_rag_storage_provider,
        Err(error) => return Err(error),
    };

    Ok(ChatbotConfig {
        chatbot_provider,
        chatbot_rag_storage_provider,
    })
}

fn chatbot_provider_from_env() -> Result<Option<ChatbotProvider>, ServerConfigError> {
    let provider = match optional_env("CHATBOT_PROVIDER") {
        Ok(provider) => provider,
        Err(error) => return Err(error),
    };

    match provider {
        Some(value) => chatbot_provider_from_value(value),
        None => Ok(None),
    }
}

fn chatbot_provider_from_value(
    value: String,
) -> Result<Option<ChatbotProvider>, ServerConfigError> {
    match normalized_env_value(&value).as_str() {
        "" | "none" | "disabled" => Ok(None),
        "claude" | "anthropic" => claude_chatbot_provider_from_env(),
        "mistral" => mistral_chatbot_provider_from_env(),
        _ => Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key: "CHATBOT_PROVIDER",
            value,
            expected: "none, disabled, claude, anthropic, or mistral",
        }),
    }
}

fn claude_chatbot_provider_from_env() -> Result<Option<ChatbotProvider>, ServerConfigError> {
    let anthropic_api_key = match required_env("ANTHROPIC_API_KEY") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let anthropic_model = match required_env("ANTHROPIC_MODEL") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let anthropic_max_tokens = match required_int_env("ANTHROPIC_MAX_TOKENS") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    Ok(Some(ChatbotProvider::Claude(ClaudeConfig::public_api(
        anthropic_api_key,
        anthropic_model,
        anthropic_max_tokens,
    ))))
}

fn mistral_chatbot_provider_from_env() -> Result<Option<ChatbotProvider>, ServerConfigError> {
    let mistral_api_key = match required_env("MISTRAL_API_KEY") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let mistral_model = match required_env("MISTRAL_MODEL") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    Ok(Some(ChatbotProvider::Mistral(MistralConfig::public_api(
        mistral_api_key,
        mistral_model,
    ))))
}

fn rag_storage_provider_from_env() -> Result<Option<RagStorageProvider>, ServerConfigError> {
    let provider = match optional_env("CHATBOT_RAG_STORAGE_PROVIDER") {
        Ok(provider) => provider,
        Err(error) => return Err(error),
    };

    match provider {
        Some(value) => match normalized_env_value(&value).as_str() {
            "" | "none" | "disabled" => Ok(None),
            "in_ram" | "in-memory" | "memory" => Ok(Some(RagStorageProvider::InRam)),
            "pg_vector" | "pgvector" => Ok(Some(RagStorageProvider::PgVector)),
            _ => Err(ServerConfigError::InvalidEnvironmentVariable {
                env_key: "CHATBOT_RAG_STORAGE_PROVIDER",
                value,
                expected: "none, disabled, in_ram, memory, pg_vector, or pgvector",
            }),
        },
        None => Ok(None),
    }
}
