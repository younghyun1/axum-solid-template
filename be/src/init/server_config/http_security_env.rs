use super::{
    http_security_config::{CookieConfig, CookieSameSitePolicy, CorsConfig, normalize_origin},
    parsers::{normalized_env_value, optional_env},
    server_config::{DeploymentEnvironment, ServerConfigError},
};

pub(super) fn cookie_config_from_env(
    deployment_environment: DeploymentEnvironment,
    https_enabled: bool,
) -> Result<CookieConfig, ServerConfigError> {
    let default_secure = match deployment_environment {
        DeploymentEnvironment::Local | DeploymentEnvironment::Development => https_enabled,
        DeploymentEnvironment::Production | DeploymentEnvironment::ProductionDockerized => true,
    };
    let secure = match optional_bool_env("AUTH_COOKIE_SECURE", default_secure) {
        Ok(secure) => secure,
        Err(error) => return Err(error),
    };

    match deployment_environment {
        DeploymentEnvironment::Production | DeploymentEnvironment::ProductionDockerized => {
            if !secure {
                return Err(ServerConfigError::InvalidEnvironmentVariable {
                    env_key: "AUTH_COOKIE_SECURE",
                    value: "false".to_string(),
                    expected: "true in production environments",
                });
            }
        }
        DeploymentEnvironment::Local | DeploymentEnvironment::Development => {}
    }

    Ok(CookieConfig {
        secure,
        same_site: CookieSameSitePolicy::Lax,
    })
}

pub(super) fn cors_config_from_env(
    deployment_environment: DeploymentEnvironment,
    public_app_base_url: &str,
    server_port: u16,
) -> Result<CorsConfig, ServerConfigError> {
    let allowed_origins = match optional_env("CORS_ALLOWED_ORIGINS") {
        Ok(Some(value)) => match parse_origin_list("CORS_ALLOWED_ORIGINS", &value) {
            Ok(origins) => origins,
            Err(error) => return Err(error),
        },
        Ok(None) => match default_origins(deployment_environment, public_app_base_url, server_port)
        {
            Ok(origins) => origins,
            Err(error) => return Err(error),
        },
        Err(error) => return Err(error),
    };

    Ok(CorsConfig {
        allowed_origins,
        allow_credentials: true,
    })
}

fn default_origins(
    deployment_environment: DeploymentEnvironment,
    public_app_base_url: &str,
    server_port: u16,
) -> Result<Vec<String>, ServerConfigError> {
    let public_origin = match normalize_origin(public_app_base_url) {
        Some(origin) => origin,
        None => {
            return Err(ServerConfigError::InvalidEnvironmentVariable {
                env_key: "PUBLIC_APP_BASE_URL",
                value: public_app_base_url.to_string(),
                expected: "an http or https origin without a path",
            });
        }
    };

    let mut origins = vec![public_origin];
    match deployment_environment {
        DeploymentEnvironment::Local | DeploymentEnvironment::Development => {
            push_origin(&mut origins, format!("http://127.0.0.1:{server_port}"));
            push_origin(&mut origins, format!("http://localhost:{server_port}"));
            push_origin(&mut origins, "http://127.0.0.1:5173".to_string());
            push_origin(&mut origins, "http://localhost:5173".to_string());
        }
        DeploymentEnvironment::Production | DeploymentEnvironment::ProductionDockerized => {}
    }

    Ok(origins)
}

fn parse_origin_list(env_key: &'static str, value: &str) -> Result<Vec<String>, ServerConfigError> {
    let mut origins = Vec::new();
    for raw_origin in value.split(',') {
        let origin = match normalize_origin(raw_origin) {
            Some(origin) => origin,
            None => {
                return Err(ServerConfigError::InvalidEnvironmentVariable {
                    env_key,
                    value: raw_origin.trim().to_string(),
                    expected: "comma-separated http or https origins without paths",
                });
            }
        };
        push_origin(&mut origins, origin);
    }

    if origins.is_empty() {
        return Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key,
            value: value.to_string(),
            expected: "at least one allowed origin",
        });
    }

    Ok(origins)
}

fn push_origin(origins: &mut Vec<String>, origin: String) {
    if !origins.iter().any(|existing| existing == &origin) {
        origins.push(origin);
    }
}

fn optional_bool_env(
    env_key: &'static str,
    default_value: bool,
) -> Result<bool, ServerConfigError> {
    let value = match optional_env(env_key) {
        Ok(Some(value)) => value,
        Ok(None) => return Ok(default_value),
        Err(error) => return Err(error),
    };

    match normalized_env_value(&value).as_str() {
        "true" | "1" | "yes" | "y" => Ok(true),
        "false" | "0" | "no" | "n" => Ok(false),
        _ => Err(ServerConfigError::InvalidEnvironmentVariable {
            env_key,
            value,
            expected: "true, false, 1, 0, yes, or no",
        }),
    }
}
