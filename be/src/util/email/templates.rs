use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn email_verification_html(verification_token: Uuid, verify_by: DateTime<Utc>) -> String {
    format!(
        r#"<!doctype html>
<html>
<body>
<h1>Verify your email</h1>
<p>Your verification token is:</p>
<pre>{verification_token}</pre>
<p>This token expires at {verify_by}.</p>
</body>
</html>"#
    )
}

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
