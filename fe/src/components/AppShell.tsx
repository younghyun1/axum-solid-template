import { A } from "@solidjs/router";
import { type Component, type JSX } from "solid-js";

interface AppShellProps {
  children?: JSX.Element;
}

export const AppShell: Component<AppShellProps> = (props) => {
  return (
    <div class="min-h-screen bg-bone text-ink">
      <div class="absolute inset-0 -z-10 bg-[radial-gradient(circle_at_15%_15%,rgb(214_90_49_/_0.22),transparent_28%),linear-gradient(135deg,#fff8eb_0%,#eadcc8_100%)]" />
      <header class="mx-auto flex w-full max-w-6xl items-center justify-between px-5 py-5 sm:px-8">
        <A class="font-sans text-sm font-black uppercase tracking-[0.26em] text-copper" href="/">
          Rust Solid
        </A>
        <nav aria-label="Primary navigation" class="flex items-center gap-2 text-sm font-bold">
          <A
            activeClass="bg-ink text-bone"
            class="rounded-full px-4 py-2 text-ink transition hover:bg-ink/10"
            end
            href="/"
          >
            Home
          </A>
          <A
            activeClass="bg-ink text-bone"
            class="rounded-full px-4 py-2 text-ink transition hover:bg-ink/10"
            href="/health"
          >
            API Health
          </A>
        </nav>
      </header>
      <main class="mx-auto w-full max-w-6xl px-5 pb-12 pt-6 sm:px-8">{props.children}</main>
    </div>
  );
};

