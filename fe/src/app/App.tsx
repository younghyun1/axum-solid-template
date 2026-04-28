import { createSignal, For } from "solid-js";

import { apiDemos } from "../api/backendApi";
import { EndpointCard } from "../components/EndpointCard";

export function App() {
  const [token, setToken] = createSignal("");

  return (
    <main class="app-shell">
      <section class="app-header">
        <div>
          <p class="eyebrow">Rust backend</p>
          <h1>API Demo Console</h1>
        </div>
        <label class="token-control">
          <span>Bearer token</span>
          <input
            autocomplete="off"
            inputmode="text"
            onInput={(event) => setToken(event.currentTarget.value)}
            placeholder="Paste login access_token"
            spellcheck={false}
            type="text"
            value={token()}
          />
        </label>
      </section>

      <section class="endpoint-grid" aria-label="API endpoints">
        <For each={apiDemos}>
          {(demo) => <EndpointCard demo={demo} token={token} onToken={setToken} />}
        </For>
      </section>
    </main>
  );
}
