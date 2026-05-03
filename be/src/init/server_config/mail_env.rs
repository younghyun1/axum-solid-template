use super::{
    mail_config::MailConfig,
    parsers::{normalized_env_value, optional_env, optional_int_env, required_env},
    server_config::ServerConfigError,
};

/// Perform the `mail_config_from_env` operation as implemented by this function.
///
/// # Arguments
/// * `super) fn mail_config_from_env(` -
/// # Returns
/// A `Result`, either containing the function output or an error.
pub(super) fn mail_config_from_env() -> Result<MailConfig, ServerConfigError> {
    let provider = match optional_env("MAIL_PROVIDER") {
        Ok(Some(provider)) => provider,
        Ok(None) => return Ok(MailConfig::disabled()),
        Err(error) => return Err(error),
    };

    match normalized_env_value(&provider).as_str() {
        "" | "none" | "disabled" => Ok(MailConfig::disabled()),
        "aws_ses" | "aws-ses" | "ses" | "aws_ses_smtp" | "ses_smtp" => {
            aws_ses_smtp_config_from_env()
        }
        _ => Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key: "MAIL_PROVIDER",
            value: provider,
            expected: "none, disabled, aws_ses, aws_ses_smtp, ses, or ses_smtp",
        }),
    }
}

/// Perform the `aws_ses_smtp_config_from_env` operation as implemented by this function.
///
/// # Returns
/// A `Result`, either containing the function output or an error.
fn aws_ses_smtp_config_from_env() -> Result<MailConfig, ServerConfigError> {
    let from_email = match required_env("MAIL_FROM_EMAIL") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let from_name = match required_env("MAIL_FROM_NAME") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let smtp_host = match required_env("AWS_SES_SMTP_HOST") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let smtp_port = match optional_int_env("AWS_SES_SMTP_PORT", 587_u16) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let smtp_username = match required_env("AWS_SES_SMTP_USERNAME") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let smtp_password = match required_env("AWS_SES_SMTP_PASSWORD") {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    Ok(MailConfig::aws_ses_smtp(
        from_email,
        from_name,
        smtp_host,
        smtp_port,
        smtp_username,
        smtp_password,
    ))
}
