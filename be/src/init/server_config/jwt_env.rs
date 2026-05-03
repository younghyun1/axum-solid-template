use std::time::Duration;

use crate::init::server_config::jwt_config::jwt_config::{JwtConfig, JwtIssuer, JwtSecretKey};

use super::{
    parsers::{required_env, required_int_env},
    server_config::ServerConfigError,
};

/// Perform the `jwt_config_from_env` operation as implemented by this function.
///
/// # Arguments
/// * `super) fn jwt_config_from_env(` -
/// # Returns
/// A `Result`, either containing the function output or an error.
pub(super) fn jwt_config_from_env() -> Result<JwtConfig, ServerConfigError> {
    let secret_key_value = match required_env("JWT_SECRET_KEY") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let secret_key = match JwtSecretKey::new(secret_key_value.into_bytes()) {
        Ok(secret_key) => secret_key,
        Err(error) => return Err(ServerConfigError::JwtConfig(error)),
    };
    let issuer = match required_env("JWT_ISSUER") {
        Ok(value) => JwtIssuer(value),
        Err(error) => return Err(error),
    };
    let access_token_seconds = match required_int_env("JWT_ACCESS_TOKEN_DURATION_SECONDS") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let refresh_token_seconds = match required_int_env("JWT_REFRESH_TOKEN_DURATION_SECONDS") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    let jwt_config = JwtConfig {
        issuer,
        access_token_duration: Duration::from_secs(access_token_seconds),
        refresh_token_duration: Duration::from_secs(refresh_token_seconds),
        secret_key,
    };

    match jwt_config.validate() {
        Ok(()) => Ok(jwt_config),
        Err(error) => Err(ServerConfigError::JwtConfig(error)),
    }
}
