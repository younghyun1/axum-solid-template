import { createMemo, createSignal, Show } from "solid-js";

import { requestPasswordReset, resetPassword } from "../../api/appApi";
import { NoticeView } from "../shared/Feedback";
import { emptyNotice, type LinkTokens, type Notice } from "../shared/types";

interface RecoveryPageProps {
  readonly linkTokens: LinkTokens;
  readonly onSignIn: () => void;
}

export function RecoveryPage(props: RecoveryPageProps) {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");
  const [notice, setNotice] = createSignal<Notice>(emptyNotice);
  const [running, setRunning] = createSignal(false);

  const passwordsMismatch = createMemo(
    () => password().length > 0 && confirmPassword().length > 0 && password() !== confirmPassword()
  );

  const requestReset = async (event: SubmitEvent) => {
    event.preventDefault();
    setRunning(true);
    const result = await requestPasswordReset({ user_email: email().trim() });
    setRunning(false);
    setNotice(
      result.ok
        ? { kind: "success", text: "Password reset email sent." }
        : { kind: "error", text: result.error.message }
    );
  };

  const applyReset = async (event: SubmitEvent) => {
    event.preventDefault();
    const token = props.linkTokens.resetToken;
    if (token === null) {
      setNotice({ kind: "error", text: "Open the reset link from your email to change password." });
      return;
    }

    if (passwordsMismatch()) {
      setNotice({ kind: "error", text: "Passwords do not match." });
      return;
    }

    setRunning(true);
    const result = await resetPassword({
      new_password: password(),
      password_reset_token: token
    });
    setRunning(false);
    setNotice(
      result.ok
        ? { kind: "success", text: "Password changed. You can now sign in." }
        : { kind: "error", text: result.error.message }
    );
  };

  return (
    <section class="page-view auth-page recovery-page">
      <div class="auth-card auth-card--narrow">
        <p class="eyebrow">Recovery</p>
        <h1>Reset password</h1>
        <Show
          when={props.linkTokens.resetToken !== null}
          fallback={
            <form class="flow-form" onSubmit={requestReset}>
              <p class="form-copy">Enter your email and we will send a reset link.</p>
              <input
                aria-label="Email"
                autocomplete="email"
                placeholder="Email"
                required
                type="email"
                value={email()}
                onInput={(event) => setEmail(event.currentTarget.value)}
              />
              <NoticeView notice={notice()} />
              <button class="primary-button" disabled={running()} type="submit">
                {running() ? "Sending" : "Send reset link"}
              </button>
              <button class="secondary-button" type="button" onClick={props.onSignIn}>
                Back to sign in
              </button>
            </form>
          }
        >
          <form class="flow-form" onSubmit={applyReset}>
            <p class="form-copy">Choose a new password for your account.</p>
            <input
              aria-label="New password"
              autocomplete="new-password"
              placeholder="New password"
              required
              type="password"
              value={password()}
              onInput={(event) => setPassword(event.currentTarget.value)}
            />
            <input
              aria-invalid={passwordsMismatch() ? "true" : "false"}
              aria-label="Re-enter new password"
              autocomplete="new-password"
              placeholder="Re-enter new password"
              required
              type="password"
              value={confirmPassword()}
              onInput={(event) => setConfirmPassword(event.currentTarget.value)}
            />
            <Show when={passwordsMismatch()}>
              <p class="field-note field-note--error">Passwords do not match.</p>
            </Show>
            <NoticeView notice={notice()} />
            <button class="primary-button" disabled={running() || passwordsMismatch()} type="submit">
              {running() ? "Changing password" : "Change password"}
            </button>
            <button class="secondary-button" type="button" onClick={props.onSignIn}>
              Back to sign in
            </button>
          </form>
        </Show>
      </div>
    </section>
  );
}
