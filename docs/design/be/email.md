# Email Design

Email delivery uses `lettre`.

Configured providers:

- `disabled`
- `aws_ses_smtp`

AWS SES is integrated through the SES SMTP interface, not the AWS SDK. Runtime mail delivery is owned by `MailSender` in `be/src/util/email/sender.rs` and stored on `ServerState`.

Configuration keys:

- `MAIL_PROVIDER`
- `MAIL_FROM_EMAIL`
- `MAIL_FROM_NAME`
- `AWS_SES_SMTP_HOST`
- `AWS_SES_SMTP_PORT`
- `AWS_SES_SMTP_USERNAME`
- `AWS_SES_SMTP_PASSWORD`

When mail is disabled, sends are logged and treated as successful. Signup queues an email verification message. Password reset requests queue a reset token message.

Email verification messages link to the frontend verification page using `PUBLIC_APP_BASE_URL` and
`/verify-email?email_validation_token_id=<uuid>`. The backend no longer sends the raw verification
token as the primary email content.

Current templates live in `be/src/util/email/templates.rs`; service-specific branding should be
expanded later.
