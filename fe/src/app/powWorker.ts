import { sha256 } from "./powSha256";

export interface PowWorkerRequest {
  readonly challengeId: string;
  readonly salt: string;
  readonly difficultyBits: number;
}

export type PowWorkerResponse =
  | {
      readonly kind: "progress";
      readonly attempts: number;
    }
  | {
      readonly kind: "solved";
      readonly nonce: string;
      readonly attempts: number;
      readonly elapsedMs: number;
    }
  | {
      readonly kind: "error";
      readonly message: string;
    };

const workerSelf = self as unknown as {
  readonly performance: Performance;
  addEventListener: (
    type: "message",
    listener: (event: MessageEvent<PowWorkerRequest>) => void
  ) => void;
  postMessage: (message: PowWorkerResponse) => void;
};

workerSelf.addEventListener("message", (event) => {
  void solve(event.data).catch((error: unknown) => {
    workerSelf.postMessage({
      kind: "error",
      message: error instanceof Error ? error.message : "Proof-of-work failed."
    });
  });
});

async function solve(request: PowWorkerRequest): Promise<void> {
  if (request.difficultyBits < 0 || request.difficultyBits > 256) {
    workerSelf.postMessage({ kind: "error", message: "Invalid proof difficulty." });
    return;
  }

  const encoder = new TextEncoder();
  const startedAt = workerSelf.performance.now();
  let nonce = 0;
  workerSelf.postMessage({ attempts: nonce, kind: "progress" });

  while (true) {
    const input = `${request.challengeId}:${request.salt}:${nonce.toString()}`;
    const bytes = sha256(encoder.encode(input));
    if (hasLeadingZeroBits(bytes, request.difficultyBits)) {
      workerSelf.postMessage({
        attempts: nonce + 1,
        elapsedMs: Math.round(workerSelf.performance.now() - startedAt),
        kind: "solved",
        nonce: nonce.toString()
      });
      return;
    }

    nonce += 1;
    if (nonce % 25 === 0) {
      workerSelf.postMessage({ attempts: nonce, kind: "progress" });
    }
  }
}

function hasLeadingZeroBits(bytes: Uint8Array, difficultyBits: number): boolean {
  let remainingBits = difficultyBits;

  for (const byte of bytes) {
    if (remainingBits === 0) {
      return true;
    }

    if (remainingBits >= 8) {
      if (byte !== 0) {
        return false;
      }
      remainingBits -= 8;
      continue;
    }

    const mask = 0xff << (8 - remainingBits);
    return (byte & mask) === 0;
  }

  return remainingBits === 0;
}
