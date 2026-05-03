use crate::init::server_config::{db_config::DatabaseConfig, file_store_config::FileStoreConfig};
use std::net::{IpAddr, Ipv4Addr};

use super::{
    cert_env::cert_config_from_env,
    chatbot_env::chatbot_config_from_env,
    jwt_env::jwt_config_from_env,
    mail_env::mail_config_from_env,
    parsers::{
        normalized_env_value, optional_env, optional_int_env, optional_ip_addr_env,
        parse_database_connection_type, parse_database_type, parse_deployment_environment,
        required_bool_env, required_env, required_int_env,
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
        let cert_config = match cert_config_from_env(https_enabled) {
            Ok(cert_config) => cert_config,
            Err(error) => return Err(error),
        };
        let server_bind_ip =
            match optional_ip_addr_env("SERVER_BIND_IP", IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))) {
                Ok(server_bind_ip) => server_bind_ip,
                Err(error) => return Err(error),
            };
        let server_port = match optional_int_env("SERVER_PORT", 3000_u16) {
            Ok(server_port) => server_port,
            Err(error) => return Err(error),
        };
        let public_app_base_url = match optional_env("PUBLIC_APP_BASE_URL") {
            Ok(Some(public_app_base_url)) => public_app_base_url,
            Ok(None) => format!("http://127.0.0.1:{server_port}"),
            Err(error) => return Err(error),
        };
        let http_redirect_port = match optional_int_env("HTTP_REDIRECT_PORT", 8080_u16) {
            Ok(http_redirect_port) => http_redirect_port,
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
        let mail_config = match mail_config_from_env() {
            Ok(mail_config) => mail_config,
            Err(error) => return Err(error),
        };

        Ok(Self {
            deployment_environment,
            https_enabled,
            server_bind_ip,
            server_port,
            public_app_base_url,
            http_redirect_port,
            db_config,
            file_store_config,
            chatbot_config,
            jwt_config,
            mail_config,
            cert_config,
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
