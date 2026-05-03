import { createSignal } from "solid-js";

import { login } from "../../api/appApi";
import type { LoginResponse } from "../../api/types";
import { NoticeView } from "../shared/Feedback";
import { emptyNotice, type Notice } from "../shared/types";

interface SignInPageProps {
  readonly onForgotPassword: () => void;
  readonly onJoin: () => void;
  readonly onLogin: (response: LoginResponse) => void;
}

export function SignInPage(props: SignInPageProps) {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [notice, setNotice] = createSignal<Notice>(emptyNotice);
  const [running, setRunning] = createSignal(false);

  const submit = async (event: SubmitEvent) => {
    event.preventDefault();
    setRunning(true);
    const result = await login({
      user_email: email().trim(),
      user_password: password()
    });
    setRunning(false);

    if (!result.ok || result.data === null) {
      setNotice({
        kind: "error",
        text: result.ok ? "Login response was empty." : result.error.message
      });
      return;
    }

    props.onLogin(result.data);
  };

  return (
    <section class="page-view auth-page">
      <div class="auth-card auth-card--narrow">
        <p class="eyebrow">Welcome back</p>
        <h1>Sign in</h1>
        <form class="flow-form" onSubmit={submit}>
          <input
            aria-label="Email"
            autocomplete="email"
            placeholder="Email"
            required
            type="email"
            value={email()}
            onInput={(event) => setEmail(event.currentTarget.value)}
          />
          <input
            aria-label="Password"
            autocomplete="current-password"
            placeholder="Password"
            required
            type="password"
            value={password()}
            onInput={(event) => setPassword(event.currentTarget.value)}
          />
          <button class="link-button" type="button" onClick={props.onForgotPassword}>
            Forgot password?
          </button>
          <NoticeView notice={notice()} />
          <button class="primary-button" disabled={running()} type="submit">
            {running() ? "Signing in" : "Sign in"}
          </button>
          <button class="secondary-button" type="button" onClick={props.onJoin}>
            Create account
          </button>
        </form>
      </div>
    </section>
  );
}
