import { For, Show } from "solid-js";

import type { MarketplaceSearchResultResponse } from "../../api/marketplaceTypes";

interface ProviderSearchResultsProps {
  readonly errorMessage: string | null;
  readonly loading: boolean;
  readonly query: string;
  readonly results: readonly MarketplaceSearchResultResponse[];
  readonly onOpenProvider: (slug: string) => void;
}

export function ProviderSearchResults(props: ProviderSearchResultsProps) {
  return (
    <Show when={props.query.length >= 2}>
      <section class="directory-search-results" aria-label="Indexed search results">
        <div class="directory-search-results__bar">
          <div>
            <h2>Search matches</h2>
            <p>
              {props.loading ? "Searching profiles and published posts" : searchSummary(props.results)}
            </p>
          </div>
          <span class="marketplace-chip">Indexed content</span>
        </div>
        <Show
          when={props.errorMessage === null}
          fallback={
            <StatePanel
              title="Search unavailable"
              body={props.errorMessage ?? "Search failed."}
            />
          }
        >
          <Show
            when={!props.loading}
            fallback={
              <StatePanel
                title="Searching marketplace"
                body="Checking provider profiles and published content."
              />
            }
          >
            <Show
              when={props.results.length > 0}
              fallback={
                <StatePanel
                  title="No indexed matches"
                  body="The directory list below may still contain filtered provider matches."
                />
              }
            >
              <div class="search-result-grid">
                <For each={props.results}>
                  {(result) => (
                    <SearchResultCard result={result} onOpenProvider={props.onOpenProvider} />
                  )}
                </For>
              </div>
            </Show>
          </Show>
        </Show>
      </section>
    </Show>
  );
}

function SearchResultCard(props: {
  readonly result: MarketplaceSearchResultResponse;
  readonly onOpenProvider: (slug: string) => void;
}) {
  const providerSlug = providerSlugFromResult(props.result);
  return (
    <article class="search-result-card">
      <span class="search-result-card__kind">{kindLabel(props.result.kind)}</span>
      <h3>{props.result.title}</h3>
      <p>{props.result.subtitle}</p>
      <Show when={props.result.snippet.length > 0}>
        <p class="search-result-card__snippet">{props.result.snippet}</p>
      </Show>
      <button
        class="secondary-button"
        type="button"
        disabled={providerSlug === null}
        onClick={() => {
          if (providerSlug !== null) {
            props.onOpenProvider(providerSlug);
          }
        }}
      >
        Open result
      </button>
    </article>
  );
}

function searchSummary(results: readonly MarketplaceSearchResultResponse[]): string {
  if (results.length === 1) {
    return "1 indexed match across marketplace content";
  }

  return `${results.length} indexed matches across marketplace content`;
}

function kindLabel(kind: MarketplaceSearchResultResponse["kind"]): string {
  switch (kind) {
    case "provider":
      return "Provider";
    case "provider_blog":
      return "Provider post";
    case "central_blog":
      return "Marketplace post";
  }
}

function providerSlugFromResult(result: MarketplaceSearchResultResponse): string | null {
  if (result.kind === "provider") {
    return result.slug;
  }
  if (!result.url_path.startsWith("/providers/")) {
    return null;
  }

  const withoutPrefix = result.url_path.slice("/providers/".length);
  const [slug] = withoutPrefix.split("/");
  if (slug === undefined || slug.length === 0) {
    return null;
  }

  try {
    return decodeURIComponent(slug);
  } catch {
    return slug;
  }
}

function StatePanel(props: { readonly title: string; readonly body: string }) {
  return (
    <div class="marketplace-state" role="status">
      <strong>{props.title}</strong>
      <p>{props.body}</p>
    </div>
  );
}
