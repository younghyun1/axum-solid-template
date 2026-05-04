import type { SignupResponse } from "../../api/types";

interface SignupVerificationPendingProps {
  readonly account: SignupResponse;
  readonly onSignIn: () => void;
}

export function SignupVerificationPending(props: SignupVerificationPendingProps) {
  return (
    <div class="flow-form">
      <p class="eyebrow">Email verification</p>
      <h1>Verify your email</h1>
      <p class="form-copy">
        Account created for {props.account.user_email}. Sign-in is blocked until that email address
        is verified.
      </p>
      <p class="form-copy">
        Check your inbox for the verification message and open the link inside it. You can sign in
        after the verification step is complete.
      </p>
      <p class="field-note">
        Verification expires {formatVerificationDeadline(props.account.verify_by)}.
      </p>
      <button class="primary-button" type="button" onClick={props.onSignIn}>
        I have verified; sign in
      </button>
    </div>
  );
}

function formatVerificationDeadline(value: string): string {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return date.toLocaleString();
}
