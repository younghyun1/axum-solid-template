import type { EmailVerificationChallengeResponse } from "../api/types";
import type { PowWorkerResponse } from "./powWorker";

export type PowSolveResult =
  | {
      readonly ok: true;
      readonly nonce: string;
    }
  | {
      readonly ok: false;
      readonly message: string;
    };

export function solveProofOfWork(
  challenge: EmailVerificationChallengeResponse,
  setAttempts: (attempts: number) => void
): Promise<PowSolveResult> {
  if (typeof Worker === "undefined") {
    return Promise.resolve({
      ok: false,
      message: "Browser workers are unavailable."
    });
  }

  return new Promise((resolve) => {
    const worker = new Worker(new URL("./powWorker.ts", import.meta.url), { type: "module" });
    let finished = false;

    worker.onmessage = (event: MessageEvent<PowWorkerResponse>) => {
      const message = event.data;
      if (message.kind === "progress") {
        setAttempts(message.attempts);
        return;
      }

      if (finished) {
        return;
      }
      finished = true;
      worker.terminate();

      if (message.kind === "solved") {
        setAttempts(message.attempts);
        resolve({ nonce: message.nonce, ok: true });
        return;
      }

      resolve({ message: message.message, ok: false });
    };

    worker.onerror = (event) => {
      if (finished) {
        return;
      }
      finished = true;
      worker.terminate();
      resolve({ message: event.message, ok: false });
    };

    worker.postMessage({
      challengeId: challenge.email_verification_challenge_id,
      difficultyBits: challenge.email_verification_pow_difficulty_bits,
      salt: challenge.email_verification_pow_salt
    });
  });
}
