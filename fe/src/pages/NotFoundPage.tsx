import { A } from "@solidjs/router";
import { type Component } from "solid-js";

export const NotFoundPage: Component = () => {
  return (
    <section class="grid min-h-[calc(100vh-9rem)] place-items-center">
      <div class="max-w-xl rounded-[2rem] border border-ink/10 bg-white/60 p-8 shadow-2xl shadow-ink/15 backdrop-blur">
        <p class="mb-4 text-xs font-black uppercase tracking-[0.22em] text-copper">404</p>
        <h1 class="text-5xl font-black tracking-[-0.05em]">Route not found.</h1>
        <A class="mt-8 inline-flex rounded-full bg-ink px-5 py-3 font-bold text-bone" href="/">
          Back home
        </A>
      </div>
    </section>
  );
};

