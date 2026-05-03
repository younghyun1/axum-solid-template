use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn email_verification_html(
    verification_token: Uuid,
    verify_by: DateTime<Utc>,
    public_app_base_url: &str,
) -> String {
    let base_url = public_app_base_url.trim_end_matches('/');
    let verification_url =
        format!("{base_url}/verify-email?email_validation_token_id={verification_token}");
    format!(
        r#"<!doctype html>
<html>
<body style="margin:0;background:#f6f8fb;color:#182026;font-family:Inter,Arial,sans-serif;">
<div style="max-width:560px;margin:0 auto;padding:32px 20px;">
<div style="border:1px solid #dce4ec;border-radius:10px;background:#ffffff;padding:26px;">
<p style="margin:0 0 10px;color:#8a4d00;font-size:12px;font-weight:700;letter-spacing:.06em;text-transform:uppercase;">Rust-Solid-Template</p>
<h1 style="margin:0 0 12px;font-size:26px;line-height:1.2;">Verify your email</h1>
<p style="margin:0 0 20px;color:#5b6873;font-size:16px;line-height:1.5;">Open the verification page and complete the short check to activate your account.</p>
<a href="{verification_url}" style="display:inline-block;min-height:40px;line-height:40px;border-radius:6px;background:#111827;color:#ffffff;padding:0 16px;text-decoration:none;font-weight:700;">Verify email</a>
<p style="margin:20px 0 0;color:#5b6873;font-size:14px;line-height:1.5;">This link expires at {verify_by}.</p>
</div>
</div>
</body>
</html>"#
    )
}

/// Perform the `password_reset_html` operation as implemented by this function.
///
/// # Arguments
/// * `password_reset_token` -
/// * `verify_by` -
/// # Returns
/// Returns the value produced by this function.
pub fn password_reset_html(password_reset_token: Uuid, verify_by: DateTime<Utc>) -> String {
    format!(
        r#"<!doctype html>
<html>
<body>
<h1>Reset your password</h1>
<p>Your password reset token is:</p>
<pre>{password_reset_token}</pre>
<p>This token expires at {verify_by}.</p>
</body>
</html>"#
    )
}
