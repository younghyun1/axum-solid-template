import { createEffect, createMemo, createSignal, onCleanup } from "solid-js";

const RESET_HOLD_DURATION_MS = 5000;

interface AdminDatabasePanelProps {
  readonly running: boolean;
  readonly onResetDatabase: () => void;
}

export function AdminDatabasePanel(props: AdminDatabasePanelProps) {
  const [holding, setHolding] = createSignal(false);
  const [committed, setCommitted] = createSignal(false);
  const [progress, setProgress] = createSignal(0);
  let animationFrameId: number | null = null;
  let activePointerId: number | null = null;
  let holdStartedAt = 0;

  const progressPercent = createMemo(() => Math.round(progress() * 100));
  const secondsRemaining = createMemo(() =>
    Math.max(0, Math.ceil((RESET_HOLD_DURATION_MS * (1 - progress())) / 1000))
  );
  const buttonLabel = createMemo(() => {
    if (props.running) {
      return "Resetting database";
    }
    if (committed()) {
      return "Reset launched";
    }
    if (holding()) {
      return `Keep holding ${secondsRemaining().toString()}s`;
    }

    return "Hold 5 seconds to reset database";
  });

  createEffect(() => {
    if (props.running || !committed()) {
      return;
    }

    const timeoutId = window.setTimeout(() => {
      setCommitted(false);
      setProgress(0);
    }, 700);
    onCleanup(() => window.clearTimeout(timeoutId));
  });

  onCleanup(() => stopAnimation());

  const stopAnimation = () => {
    if (animationFrameId === null) {
      return;
    }

    window.cancelAnimationFrame(animationFrameId);
    animationFrameId = null;
  };

  const completeHold = () => {
    stopAnimation();
    activePointerId = null;
    setHolding(false);
    setCommitted(true);
    setProgress(1);
    props.onResetDatabase();
  };

  const animateHold = (now: number) => {
    const nextProgress = Math.min(1, (now - holdStartedAt) / RESET_HOLD_DURATION_MS);
    setProgress(nextProgress);
    if (nextProgress >= 1) {
      completeHold();
      return;
    }

    animationFrameId = window.requestAnimationFrame(animateHold);
  };

  const startHold = () => {
    if (props.running || committed() || holding()) {
      return;
    }

    holdStartedAt = window.performance.now();
    setProgress(0);
    setHolding(true);
    stopAnimation();
    animationFrameId = window.requestAnimationFrame(animateHold);
  };

  const cancelHold = () => {
    if (committed() || props.running) {
      return;
    }

    stopAnimation();
    activePointerId = null;
    setHolding(false);
    setProgress(0);
  };

  const handlePointerDown = (event: PointerEvent) => {
    if (event.button !== 0) {
      return;
    }

    if (!(event.currentTarget instanceof HTMLButtonElement)) {
      return;
    }

    activePointerId = event.pointerId;
    event.currentTarget.setPointerCapture(event.pointerId);
    startHold();
  };

  const handlePointerEnd = (event: PointerEvent) => {
    if (!(event.currentTarget instanceof HTMLButtonElement)) {
      cancelHold();
      return;
    }

    if (activePointerId !== null && event.currentTarget.hasPointerCapture(activePointerId)) {
      event.currentTarget.releasePointerCapture(activePointerId);
    }
    cancelHold();
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key !== " " && event.key !== "Enter") {
      return;
    }

    event.preventDefault();
    startHold();
  };

  const handleKeyUp = (event: KeyboardEvent) => {
    if (event.key !== " " && event.key !== "Enter") {
      return;
    }

    event.preventDefault();
    cancelHold();
  };

  return (
    <section class="auth-card admin-form admin-form--narrow database-reset-panel">
      <div>
        <p class="eyebrow">Destructive operation</p>
        <h2>Database reset</h2>
      </div>
      <p class="form-copy">
        Hold the control for five seconds to run every embedded Diesel down migration, then every up
        migration. This wipes local application data.
      </p>
      <div class="database-reset-stage">
        <button
          aria-label="Hold for five seconds to reset the database"
          aria-valuemax="100"
          aria-valuemin="0"
          aria-valuenow={progressPercent()}
          class="database-reset-hold-button"
          disabled={props.running}
          style={`--reset-progress: ${(progress() * 100).toFixed(1)}%;`}
          type="button"
          onKeyDown={handleKeyDown}
          onKeyUp={handleKeyUp}
          onPointerCancel={handlePointerEnd}
          onPointerDown={handlePointerDown}
          onPointerUp={handlePointerEnd}
        >
          <span>{buttonLabel()}</span>
        </button>
      </div>
    </section>
  );
}
