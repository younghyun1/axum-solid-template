use super::{
    parsers::required_env,
    server_config::{CertConfig, ServerConfigError},
};

pub(super) fn cert_config_from_env(
    https_enabled: bool,
) -> Result<Option<CertConfig>, ServerConfigError> {
    match https_enabled {
        true => cert_config_when_https_enabled(),
        false => Ok(None),
    }
}

/// Perform the `cert_config_when_https_enabled` operation as implemented by this function.
///
/// # Returns
/// A `Result`, either containing the function output or an error.
fn cert_config_when_https_enabled() -> Result<Option<CertConfig>, ServerConfigError> {
    let cert_chain_path = match required_env("HTTPS_CERT_CHAIN_PATH") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let private_key_path = match required_env("HTTPS_PRIVATE_KEY_PATH") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    Ok(Some(CertConfig {
        cert_chain_path,
        private_key_path,
    }))
}
