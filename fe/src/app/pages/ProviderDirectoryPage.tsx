import { createMemo, createResource, createSignal, For, Show } from "solid-js";

import { getProviderDirectory } from "../../api/marketplaceApi";
import { resultData } from "../helpers";

interface ProviderDirectoryPageProps {
  readonly onOpenProvider: (slug: string) => void;
}

export function ProviderDirectoryPage(props: ProviderDirectoryPageProps) {
  const [query, setQuery] = createSignal("");
  const [serviceArea, setServiceArea] = createSignal("");
  const [filters, setFilters] = createSignal({ q: "", service_area: "" });
  const [directoryResult] = createResource(filters, getProviderDirectory);
  const providers = createMemo(() => resultData(directoryResult())?.providers ?? []);

  const applyFilters = () => {
    setFilters({
      q: query().trim(),
      service_area: serviceArea().trim()
    });
  };

  return (
    <section class="marketplace-layout">
      <div class="marketplace-heading">
        <p class="eyebrow">Provider directory</p>
        <h1>Find service providers</h1>
      </div>

      <div class="marketplace-toolbar">
        <input
          aria-label="Search providers"
          placeholder="Search by provider name"
          value={query()}
          onInput={(event) => setQuery(event.currentTarget.value)}
        />
        <input
          aria-label="Service area"
          placeholder="Service area"
          value={serviceArea()}
          onInput={(event) => setServiceArea(event.currentTarget.value)}
        />
        <button class="primary-button" type="button" onClick={applyFilters}>
          Filter
        </button>
      </div>

      <Show
        when={providers().length > 0}
        fallback={<p class="marketplace-empty">No published providers match the current filters.</p>}
      >
        <div class="provider-grid">
          <For each={providers()}>
            {(provider) => (
              <article class="provider-card">
                <div class="provider-card__image">
                  <Show when={provider.primary_image?.public_url} fallback={<span>Profile</span>}>
                    {(url) => <img src={url()} alt="" loading="lazy" />}
                  </Show>
                </div>
                <div>
                  <h2>{provider.display_name}</h2>
                  <p>{provider.headline ?? "Marketplace provider"}</p>
                </div>
                <Show when={provider.service_area}>
                  {(area) => <span class="marketplace-chip">{area()}</span>}
                </Show>
                <button
                  class="secondary-button"
                  type="button"
                  onClick={() => props.onOpenProvider(provider.slug)}
                >
                  View profile
                </button>
              </article>
            )}
          </For>
        </div>
      </Show>
    </section>
  );
}
