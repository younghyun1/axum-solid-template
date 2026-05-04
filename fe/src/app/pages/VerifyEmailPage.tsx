import { createEffect, createSignal, Show } from "solid-js";

import { issueEmailVerificationChallenge, me, verifyEmail } from "../../api/appApi";
import type { EmailVerificationChallengeResponse, MeResponse } from "../../api/types";
import { solveProofOfWork } from "../proofOfWork";
import { NoticeView, SpinnerStatus } from "../shared/Feedback";
import { emptyNotice, type LinkTokens, type Notice } from "../shared/types";

interface VerifyEmailPageProps {
  readonly isSignedIn: boolean;
  readonly linkTokens: LinkTokens;
  readonly onHome: () => void;
  readonly onProfileLoaded: (profile: MeResponse) => void;
  readonly onSignIn: () => void;
}

export function VerifyEmailPage(props: VerifyEmailPageProps) {
  const [challenge, setChallenge] = createSignal<EmailVerificationChallengeResponse | null>(null);
  const [answer, setAnswer] = createSignal("");
  const [honeypot, setHoneypot] = createSignal("");
  const [powNonce, setPowNonce] = createSignal("");
  const [powAttempts, setPowAttempts] = createSignal(0);
  const [loading, setLoading] = createSignal(false);
  const [powRunning, setPowRunning] = createSignal(false);
  const [timingReady, setTimingReady] = createSignal(false);
  const [submitting, setSubmitting] = createSignal(false);
  const [verified, setVerified] = createSignal(false);
  const [authenticated, setAuthenticated] = createSignal(props.isSignedIn);
  const [notice, setNotice] = createSignal<Notice>(emptyNotice);
  let started = false;

  createEffect(() => {
    const token = props.linkTokens.verificationToken;
    if (started) {
      return;
    }
    started = true;

    if (token === null) {
      setNotice({ kind: "error", text: "Open the verification link from your email." });
      return;
    }

    void loadChallenge(token);
  });

  const loadChallenge = async (emailValidationTokenId: string) => {
    setLoading(true);
    const challengeResult = await issueEmailVerificationChallenge(emailValidationTokenId);
    setLoading(false);

    if (!challengeResult.ok || challengeResult.data === null) {
      setNotice({
        kind: "error",
        text: challengeResult.ok ? "Challenge response was empty." : challengeResult.error.message
      });
      return;
    }

    setChallenge(challengeResult.data);
    setTimingReady(false);
    const challengeReceivedAt = window.performance.now();
    setPowRunning(true);
    const nonceResult = await solveProofOfWork(challengeResult.data, setPowAttempts);
    setPowRunning(false);
    if (!nonceResult.ok) {
      setNotice({ kind: "error", text: nonceResult.message });
      return;
    }
    setPowNonce(nonceResult.nonce);
    await waitForMinimumElapsed(
      challengeResult.data.email_verification_minimum_elapsed_ms,
      challengeReceivedAt
    );
    setTimingReady(true);
  };

  const submit = async (event: SubmitEvent) => {
    event.preventDefault();
    const emailValidationTokenId = props.linkTokens.verificationToken;
    const currentChallenge = challenge();
    if (
      emailValidationTokenId === null ||
      currentChallenge === null ||
      powNonce().length === 0 ||
      !timingReady()
    ) {
      setNotice({ kind: "error", text: "Verification challenge is not ready." });
      return;
    }

    setSubmitting(true);
    const result = await verifyEmail({
      email_validation_token_id: emailValidationTokenId,
      email_verification_answer: answer(),
      email_verification_challenge_id: currentChallenge.email_verification_challenge_id,
      email_verification_honeypot: honeypot(),
      email_verification_pow_nonce: powNonce()
    });
    setSubmitting(false);

    if (!result.ok) {
      setNotice({ kind: "error", text: result.error.message });
      return;
    }

    setVerified(true);
    setNotice({ kind: "success", text: "Email verified." });
    if (props.isSignedIn) {
      const profileResult = await me();
      if (profileResult.ok && profileResult.data !== null) {
        props.onProfileLoaded(profileResult.data);
        setAuthenticated(true);
      }
    }
  };

  return (
    <section class="page-view auth-page">
      <div class="auth-card verification-card">
        <p class="eyebrow">Email verification</p>
        <h1>Verify your email</h1>
        <Show
          when={!verified()}
          fallback={
            <div class="flow-form">
              <p class="form-copy">
                {authenticated()
                  ? "Your email is verified and your signed-in profile has been refreshed."
                  : "Your email is verified. Sign in to continue with the verified account."}
              </p>
              <NoticeView notice={notice()} />
              <button class="primary-button" type="button" onClick={props.onHome}>
                Home
              </button>
              <Show when={!authenticated()}>
                <button class="secondary-button" type="button" onClick={props.onSignIn}>
                  Sign in
                </button>
              </Show>
            </div>
          }
        >
          <form class="flow-form" onSubmit={submit}>
            <Show when={loading()}>
              <SpinnerStatus text="Preparing verification challenge" />
            </Show>
            <Show when={powRunning()}>
              <SpinnerStatus text={`Solving browser proof ${powAttempts().toString()}`} />
            </Show>
            <Show when={challenge() !== null && powNonce().length > 0 && !timingReady()}>
              <SpinnerStatus text="Preparing verification timing" />
            </Show>
            <Show when={challenge() !== null && !loading()}>
              <p class="challenge-question">{challenge()?.email_verification_question_prompt}</p>
              <input
                aria-label="Verification answer"
                autocomplete="off"
                disabled={submitting()}
                placeholder="Answer"
                required
                value={answer()}
                onInput={(event) => setAnswer(event.currentTarget.value)}
              />
              <label class="verification-trap">
                <span>Leave this field empty</span>
                <input
                  autocomplete="off"
                  tabIndex={-1}
                  value={honeypot()}
                  onInput={(event) => setHoneypot(event.currentTarget.value)}
                />
              </label>
            </Show>
            <NoticeView notice={notice()} />
            <button
              class="primary-button"
              disabled={submitting() || powRunning() || powNonce().length === 0 || !timingReady()}
              type="submit"
            >
              {submitting() ? "Verifying" : "Verify email"}
            </button>
            <button class="secondary-button" type="button" onClick={props.onHome}>
              Home
            </button>
          </form>
        </Show>
      </div>
    </section>
  );
}

function waitForMinimumElapsed(minimumElapsedMs: number, challengeReceivedAt: number): Promise<void> {
  const remainingMs = Math.max(0, minimumElapsedMs - (window.performance.now() - challengeReceivedAt));
  if (remainingMs <= 0) {
    return Promise.resolve();
  }

  return new Promise((resolve) => {
    window.setTimeout(resolve, remainingMs);
  });
}
