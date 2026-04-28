use std::sync::Arc;

use chrono::{DateTime, Utc};
use tracing::error;
use uuid::Uuid;

use crate::{
    init::state::server_state::ServerState,
    util::email::templates::{email_verification_html, password_reset_html},
};

pub fn queue_verification_email(
    state: Arc<ServerState>,
    user_email: String,
    token: Uuid,
    verify_by: DateTime<Utc>,
) {
    tokio::spawn(async move {
        let html = email_verification_html(token, verify_by);
        match state
            .mail_sender
            .send_html(&user_email, "Verify your email", html)
            .await
        {
            Ok(()) => {}
            Err(error) => {
                error!(error = %error, user_email = %user_email, "Failed to send verification email");
            }
        }
    });
}

pub fn queue_password_reset_email(
    state: Arc<ServerState>,
    user_email: String,
    token: Uuid,
    verify_by: DateTime<Utc>,
) {
    tokio::spawn(async move {
        let html = password_reset_html(token, verify_by);
        match state
            .mail_sender
            .send_html(&user_email, "Reset your password", html)
            .await
        {
            Ok(()) => {}
            Err(error) => {
                error!(error = %error, user_email = %user_email, "Failed to send password reset email");
            }
        }
    });
}
