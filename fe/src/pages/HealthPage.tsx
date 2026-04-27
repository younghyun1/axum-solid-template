import { createResource, Match, Switch, type Component } from "solid-js";
import { backendService } from "../services/backend";

export const HealthPage: Component = () => {
  const [health] = createResource(() => backendService.health());

  return (
    <section class="grid gap-6 py-10">
      <div>
        <p class="mb-3 text-xs font-black uppercase tracking-[0.22em] text-copper">
          Axum bridge
        </p>
        <h1 class="text-4xl font-black tracking-[-0.04em] md:text-6xl">API health</h1>
      </div>

      <div class="rounded-[1.75rem] border border-ink/10 bg-white/60 p-6 shadow-xl shadow-ink/10 backdrop-blur">
        <Switch>
          <Match when={health.loading}>
            <p class="font-bold text-ink/70">Checking backend...</p>
          </Match>
          <Match when={health()}>
            {(result) => (
              <Switch>
                <Match when={result().ok}>
                  <div class="grid gap-3">
                    <p class="text-lg font-black text-rust">Backend reachable.</p>
                    <pre class="overflow-auto rounded-2xl bg-ink p-4 text-sm text-bone">
                      {JSON.stringify(result(), null, 2)}
                    </pre>
                  </div>
                </Match>
                <Match when={!result().ok}>
                  <div class="grid gap-3">
                    <p class="text-lg font-black text-copper">Backend unavailable.</p>
                    <pre class="overflow-auto rounded-2xl bg-ink p-4 text-sm text-bone">
                      {JSON.stringify(result(), null, 2)}
                    </pre>
                  </div>
                </Match>
              </Switch>
            )}
          </Match>
        </Switch>
      </div>
    </section>
  );
};

