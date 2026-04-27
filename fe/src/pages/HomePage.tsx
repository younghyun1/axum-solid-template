import { createSignal, type Component } from "solid-js";

export const HomePage: Component = () => {
  const [count, setCount] = createSignal<number>(0);

  const increment = (): void => {
    setCount((value) => value + 1);
  };

  return (
    <section class="grid min-h-[calc(100vh-9rem)] place-items-center">
      <div class="max-w-3xl rounded-[2rem] border border-ink/10 bg-white/55 p-8 shadow-2xl shadow-ink/15 backdrop-blur md:p-14">
        <p class="mb-4 text-xs font-black uppercase tracking-[0.22em] text-copper">
          SolidJS 1 + Vite 8 + TypeScript 6 + Tailwind 4
        </p>
        <h1 class="text-5xl font-black leading-[0.92] tracking-[-0.05em] text-ink md:text-7xl">
          Frontend initialized.
        </h1>
        <p class="mt-6 max-w-2xl text-lg leading-8 text-ink/70">
          The app now has a small route layer and a typed backend boundary ready
          for Axum JSON endpoints.
        </p>
        <div class="mt-8 flex flex-wrap items-center gap-4">
          <button
            class="rounded-full bg-ink px-5 py-3 font-bold text-bone transition hover:-translate-y-0.5 hover:bg-copper focus:outline-none focus:ring-4 focus:ring-rust/40"
            type="button"
            onClick={increment}
          >
            Increment
          </button>
          <span class="text-base text-ink/70">
            Count: <strong class="text-rust">{count()}</strong>
          </span>
        </div>
      </div>
    </section>
  );
};

