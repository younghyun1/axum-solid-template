#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailConfig {
    pub provider: MailProvider,
    pub from_email: String,
    pub from_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MailProvider {
    Disabled,
    AwsSesSmtp(AwsSesSmtpConfig),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwsSesSmtpConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl MailConfig {
    /// Perform the `disabled` operation as implemented by this function.
    ///
    /// # Returns
    /// Returns the value produced by this function.
    pub fn disabled() -> Self {
        Self {
            provider: MailProvider::Disabled,
            from_email: "noreply@example.invalid".to_string(),
            from_name: "rust-solid-template".to_string(),
        }
    }

    pub fn aws_ses_smtp(
        from_email: String,
        from_name: String,
        smtp_host: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
    ) -> Self {
        Self {
            provider: MailProvider::AwsSesSmtp(AwsSesSmtpConfig {
                smtp_host,
                smtp_port,
                smtp_username,
                smtp_password,
            }),
            from_email,
            from_name,
        }
    }
}
