use std::{fmt, sync::Arc};

use lettre::{
    Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use tracing::{error, info};

use crate::init::server_config::mail_config::{MailConfig, MailProvider};

#[derive(Clone)]
pub struct MailSender {
    inner: Arc<MailSenderInner>,
}

enum MailSenderInner {
    Disabled,
    AwsSesSmtp {
        transport: AsyncSmtpTransport<Tokio1Executor>,
        from: Mailbox,
    },
}

#[derive(Debug)]
pub enum MailSenderError {
    Address { error: String },
    Transport { error: String },
    Send { error: String },
}

impl MailSender {
    /// Perform the `from_config` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `config` -
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    pub fn from_config(config: &MailConfig) -> Result<Self, MailSenderError> {
        match &config.provider {
            MailProvider::Disabled => Ok(Self {
                inner: Arc::new(MailSenderInner::Disabled),
            }),
            MailProvider::AwsSesSmtp(ses_config) => {
                let from = Mailbox::new(
                    Some(config.from_name.clone()),
                    match config.from_email.parse::<Address>() {
                        Ok(address) => address,
                        Err(error) => {
                            return Err(MailSenderError::Address {
                                error: error.to_string(),
                            });
                        }
                    },
                );

                let credentials = Credentials::new(
                    ses_config.smtp_username.clone(),
                    ses_config.smtp_password.clone(),
                );
                let transport_builder =
                    match AsyncSmtpTransport::<Tokio1Executor>::relay(&ses_config.smtp_host) {
                        Ok(builder) => builder,
                        Err(error) => {
                            return Err(MailSenderError::Transport {
                                error: error.to_string(),
                            });
                        }
                    };
                let transport = transport_builder
                    .port(ses_config.smtp_port)
                    .credentials(credentials)
                    .build();

                Ok(Self {
                    inner: Arc::new(MailSenderInner::AwsSesSmtp { transport, from }),
                })
            }
        }
    }

    pub async fn send_html(
        &self,
        to_email: &str,
        subject: &str,
        html_body: String,
    ) -> Result<(), MailSenderError> {
        let (transport, from) = match self.inner.as_ref() {
            MailSenderInner::Disabled => {
                info!(to_email = %to_email, subject = subject, "Mail delivery disabled");
                return Ok(());
            }
            MailSenderInner::AwsSesSmtp { transport, from } => (transport, from),
        };

        let to_address = match to_email.parse::<Address>() {
            Ok(address) => address,
            Err(error) => {
                return Err(MailSenderError::Address {
                    error: error.to_string(),
                });
            }
        };
        let message = match Message::builder()
            .from(from.clone())
            .to(Mailbox::new(None, to_address))
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body)
        {
            Ok(message) => message,
            Err(error) => {
                return Err(MailSenderError::Transport {
                    error: error.to_string(),
                });
            }
        };

        match transport.send(message).await {
            Ok(_) => Ok(()),
            Err(error) => {
                error!(error = %error, to_email = %to_email, subject = subject, "Mail delivery failed");
                Err(MailSenderError::Send {
                    error: error.to_string(),
                })
            }
        }
    }
}

impl fmt::Debug for MailSender {
    /// Perform the `fmt` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.as_ref() {
            MailSenderInner::Disabled => formatter
                .debug_struct("MailSender")
                .field("provider", &"disabled")
                .finish(),
            MailSenderInner::AwsSesSmtp { .. } => formatter
                .debug_struct("MailSender")
                .field("provider", &"aws_ses_smtp")
                .finish(),
        }
    }
}

impl fmt::Display for MailSenderError {
    /// Perform the `fmt` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Address { error } => write!(formatter, "invalid email address: {error}"),
            Self::Transport { error } => write!(formatter, "mail transport error: {error}"),
            Self::Send { error } => write!(formatter, "mail send error: {error}"),
        }
    }
}
