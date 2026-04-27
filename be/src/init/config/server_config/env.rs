use crate::init::config::{
    chatbot::{
        chatbot_config::{ChatbotConfig, ChatbotProvider, RagStorageProvider},
        claude::ClaudeConfig,
        mistral::MistralConfig,
    },
    db_config::DatabaseConfig,
    file_store_config::FileStoreConfig,
};

use super::{
    jwt_env::jwt_config_from_env,
    parsers::{
        normalized_env_value, optional_env, parse_database_connection_type, parse_database_type,
        parse_deployment_environment, required_bool_env, required_env, required_int_env,
    },
    server_config::{
        DEPLOYMENT_ENVIRONMENT_KEY, DeploymentEnvironment, ServerConfig, ServerConfigError,
    },
};

impl ServerConfig {
    pub fn from_env() -> Result<Self, ServerConfigError> {
        let deployment_environment = match deployment_environment_from_env() {
            Ok(deployment_environment) => deployment_environment,
            Err(error) => return Err(error),
        };
        let https_enabled = match required_bool_env("HTTPS_ENABLED") {
            Ok(https_enabled) => https_enabled,
            Err(error) => return Err(error),
        };
        let db_config = match database_config_from_env() {
            Ok(db_config) => db_config,
            Err(error) => return Err(error),
        };
        let file_store_config = match file_store_config_from_env() {
            Ok(file_store_config) => file_store_config,
            Err(error) => return Err(error),
        };
        let chatbot_config = match chatbot_config_from_env() {
            Ok(chatbot_config) => chatbot_config,
            Err(error) => return Err(error),
        };
        let jwt_config = match jwt_config_from_env() {
            Ok(jwt_config) => jwt_config,
            Err(error) => return Err(error),
        };

        Ok(Self {
            deployment_environment,
            https_enabled,
            db_config,
            file_store_config,
            chatbot_config,
            jwt_config,
        })
    }
}

fn deployment_environment_from_env() -> Result<DeploymentEnvironment, ServerConfigError> {
    let value = match required_env(DEPLOYMENT_ENVIRONMENT_KEY) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    parse_deployment_environment(value)
}

fn database_config_from_env() -> Result<DatabaseConfig, ServerConfigError> {
    let database_type_value = match required_env("DATABASE_TYPE") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let database_type = match parse_database_type(database_type_value) {
        Ok(database_type) => database_type,
        Err(error) => return Err(error),
    };
    let database_connection_type_value = match required_env("DATABASE_CONNECTION_TYPE") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let database_connection_type =
        match parse_database_connection_type(database_connection_type_value) {
            Ok(database_connection_type) => database_connection_type,
            Err(error) => return Err(error),
        };
    let database_host = match required_env("DATABASE_HOST") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let database_port = match required_int_env("DATABASE_PORT") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let database_username = match required_env("DATABASE_USERNAME") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let database_password = match required_env("DATABASE_PASSWORD") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let database_name = match required_env("DATABASE_NAME") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    Ok(DatabaseConfig::new(
        database_type,
        database_connection_type,
        database_host,
        database_port,
        database_username,
        database_password,
        database_name,
    ))
}

fn file_store_config_from_env() -> Result<FileStoreConfig, ServerConfigError> {
    let file_store_type = match required_env("FILE_STORE_TYPE") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    match normalized_env_value(&file_store_type).as_str() {
        "local" => local_file_store_config_from_env(),
        "aws_s3" | "s3" => aws_s3_file_store_config_from_env(),
        _ => Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key: "FILE_STORE_TYPE",
            value: file_store_type,
            expected: "local, aws_s3, or s3",
        }),
    }
}

fn local_file_store_config_from_env() -> Result<FileStoreConfig, ServerConfigError> {
    let local_file_store_base_path = match required_env("LOCAL_FILE_STORE_BASE_PATH") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    Ok(FileStoreConfig::local(local_file_store_base_path))
}

fn aws_s3_file_store_config_from_env() -> Result<FileStoreConfig, ServerConfigError> {
    let aws_s3_bucket_name = match required_env("AWS_S3_BUCKET_NAME") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let aws_s3_access_key = match required_env("AWS_S3_ACCESS_KEY") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let aws_s3_secret_key = match required_env("AWS_S3_SECRET_KEY") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let aws_s3_region = match required_env("AWS_S3_REGION") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    Ok(FileStoreConfig::aws_s3(
        aws_s3_bucket_name,
        aws_s3_access_key,
        aws_s3_secret_key,
        aws_s3_region,
    ))
}

fn chatbot_config_from_env() -> Result<ChatbotConfig, ServerConfigError> {
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
