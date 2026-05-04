import { createMemo, createResource, createSignal, For, Show } from "solid-js";

import { getProviderDirectory, searchMarketplace } from "../../api/marketplaceApi";
import type { ProviderDirectoryCardResponse } from "../../api/marketplaceTypes";
import { resultData } from "../helpers";
import { ProviderSearchResults } from "./ProviderSearchResults";

interface ProviderDirectoryPageProps {
  readonly onOpenProvider: (slug: string) => void;
}

type ProviderSortMode = "recommended" | "name" | "area";

export function ProviderDirectoryPage(props: ProviderDirectoryPageProps) {
  const [query, setQuery] = createSignal("");
  const [serviceArea, setServiceArea] = createSignal("");
  const [sortMode, setSortMode] = createSignal<ProviderSortMode>("recommended");
  const [filters, setFilters] = createSignal({ q: "", service_area: "" });
  const [directoryResult] = createResource(filters, getProviderDirectory);
  const searchSource = createMemo(() => {
    const value = filters().q.trim();
    if (value.length < 2) {
      return undefined;
    }

    return value;
  });
  const [searchResult] = createResource(searchSource, (q) =>
    searchMarketplace({ q, limit: 8 })
  );
  const providers = createMemo(() => resultData(directoryResult())?.providers ?? []);
  const searchResults = createMemo(() => resultData(searchResult())?.results ?? []);
  const sortedProviders = createMemo(() => sortProviders(providers(), sortMode()));
  const appliedQuery = createMemo(() => filters().q);
  const appliedArea = createMemo(() => filters().service_area);
  const errorMessage = createMemo(() => {
    const result = directoryResult();
    if (result === undefined || result.ok) {
      return null;
    }

    return result.error.message;
  });

  const applyFilters = () => {
    setFilters({
      q: query().trim(),
      service_area: serviceArea().trim()
    });
  };

  const clearFilters = () => {
    setQuery("");
    setServiceArea("");
    setFilters({ q: "", service_area: "" });
  };

  return (
    <section class="public-marketplace">
      <header class="directory-header">
        <div class="directory-header__copy">
          <p class="eyebrow">Provider directory</p>
          <h1>Find verified service providers</h1>
          <p>
            Compare published provider profiles by specialty, location, media, and recent updates.
          </p>
        </div>
        <div class="directory-header__stats" aria-label="Directory summary">
          <span>{providers().length}</span>
          <p>published providers</p>
        </div>
      </header>

      <form
        class="directory-search"
        onSubmit={(event) => {
          event.preventDefault();
          applyFilters();
        }}
      >
        <label class="directory-search__field">
          <span>Search</span>
          <input
            aria-label="Search providers"
            placeholder="Name, headline, or specialty"
            value={query()}
            onInput={(event) => setQuery(event.currentTarget.value)}
          />
        </label>
        <label class="directory-search__field directory-search__field--area">
          <span>Service area</span>
          <input
            aria-label="Service area"
            placeholder="City, region, or remote"
            value={serviceArea()}
            onInput={(event) => setServiceArea(event.currentTarget.value)}
          />
        </label>
        <button class="primary-button" type="submit">
          Search
        </button>
      </form>

      <div class="directory-shell">
        <aside class="directory-filters" aria-label="Directory filters">
          <div>
            <p class="eyebrow">Filters</p>
            <h2>Refine results</h2>
          </div>
          <label class="directory-control">
            <span>Sort</span>
            <select
              value={sortMode()}
              onInput={(event) => setSortMode(event.currentTarget.value as ProviderSortMode)}
            >
              <option value="recommended">Recommended</option>
              <option value="name">Provider name</option>
              <option value="area">Service area</option>
            </select>
          </label>
          <div class="directory-filter-list">
            <span class="directory-filter-list__label">Active filters</span>
            <Show
              when={appliedQuery().length > 0 || appliedArea().length > 0}
              fallback={<p>No filters applied.</p>}
            >
              <For each={[appliedQuery(), appliedArea()].filter((value) => value.length > 0)}>
                {(value) => <span class="marketplace-chip">{value}</span>}
              </For>
            </Show>
          </div>
          <button class="secondary-button" type="button" onClick={clearFilters}>
            Clear filters
          </button>
        </aside>

        <div class="directory-results">
          <ProviderSearchResults
            errorMessage={searchErrorMessage(searchResult())}
            loading={searchResult.loading}
            query={appliedQuery()}
            results={searchResults()}
            onOpenProvider={props.onOpenProvider}
          />

          <div class="directory-results__bar">
            <div>
              <h2>Available providers</h2>
              <p>{directoryResult.loading ? "Refreshing results" : resultSummary(sortedProviders())}</p>
            </div>
            <span class="marketplace-chip">Public profiles</span>
          </div>

          <Show when={errorMessage() === null} fallback={<StatePanel title="Directory unavailable" body={errorMessage() ?? "The directory could not be loaded."} />}>
            <Show when={!directoryResult.loading} fallback={<StatePanel title="Loading providers" body="Fetching the latest published profiles." />}>
              <Show
                when={sortedProviders().length > 0}
                fallback={<StatePanel title="No matching providers" body="Adjust the search or service area filters to broaden the result set." />}
              >
                <div class="directory-list">
                  <For each={sortedProviders()}>
                    {(provider) => (
                      <article class="directory-card">
                        <div class="directory-card__media">
                          <Show
                            when={provider.primary_image?.public_url}
                            fallback={<span>{providerInitials(provider.display_name)}</span>}
                          >
                            {(url) => <img src={url()} alt="" loading="lazy" />}
                          </Show>
                        </div>
                        <div class="directory-card__body">
                          <div class="directory-card__title-row">
                            <div>
                              <h3>{provider.display_name}</h3>
                              <p>{provider.headline ?? "Published marketplace provider"}</p>
                            </div>
                            <span class="directory-card__status">Published</span>
                          </div>
                          <div class="directory-card__meta">
                            <span>{provider.service_area ?? "Service area listed on profile"}</span>
                            <span>{provider.primary_image === null ? "Media pending" : "Image verified"}</span>
                          </div>
                        </div>
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
            </Show>
          </Show>
        </div>
      </div>
    </section>
  );
}

function sortProviders(
  providers: readonly ProviderDirectoryCardResponse[],
  sortMode: ProviderSortMode
): readonly ProviderDirectoryCardResponse[] {
  const sortedProviders = [...providers];
  if (sortMode === "name") {
    return sortedProviders.sort((left, right) => left.display_name.localeCompare(right.display_name));
  }

  if (sortMode === "area") {
    return sortedProviders.sort((left, right) =>
      (left.service_area ?? "").localeCompare(right.service_area ?? "")
    );
  }

  return sortedProviders;
}

function providerInitials(displayName: string): string {
  const initials = displayName
    .split(" ")
    .filter((part) => part.length > 0)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() ?? "")
    .join("");

  if (initials.length === 0) {
    return "SP";
  }

  return initials;
}

function resultSummary(providers: readonly ProviderDirectoryCardResponse[]): string {
  if (providers.length === 1) {
    return "1 provider matches the current view";
  }

  return `${providers.length} providers match the current view`;
}

function searchErrorMessage(result: Awaited<ReturnType<typeof searchMarketplace>> | undefined): string | null {
  if (result === undefined || result.ok) {
    return null;
  }

  return result.error.message;
}

function StatePanel(props: { readonly title: string; readonly body: string }) {
  return (
    <div class="marketplace-state" role="status">
      <strong>{props.title}</strong>
      <p>{props.body}</p>
    </div>
  );
}
