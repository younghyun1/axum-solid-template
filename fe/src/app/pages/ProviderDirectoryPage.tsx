import { createMemo, createResource, createSignal } from "solid-js";

import { getProviderDirectory, searchMarketplace } from "../../api/marketplaceApi";
import { resultData } from "../helpers";
import { ProviderDirectorySearch } from "./ProviderDirectorySearch";
import { ProviderDirectorySections } from "./ProviderDirectorySections";
import { ProviderListingGrid } from "./ProviderListingGrid";
import { ProviderSearchResults } from "./ProviderSearchResults";
import {
  filterProvidersByMedia,
  resultSummary,
  sectionVisible,
  serviceAreaOptions,
  sortProviders
} from "./providerDirectoryModel";
import type {
  DirectoryFilters,
  DirectorySection,
  ProviderMediaMode,
  ProviderSortMode
} from "./providerDirectoryModel";

interface ProviderDirectoryPageProps {
  readonly onOpenProvider: (slug: string) => void;
}

export function ProviderDirectoryPage(props: ProviderDirectoryPageProps) {
  const [query, setQuery] = createSignal("");
  const [serviceArea, setServiceArea] = createSignal("");
  const [sortMode, setSortMode] = createSignal<ProviderSortMode>("recommended");
  const [mediaMode, setMediaMode] = createSignal<ProviderMediaMode>("all");
  const [activeSection, setActiveSection] = createSignal<DirectorySection>("all");
  const [filters, setFilters] = createSignal<DirectoryFilters>({ q: "", service_area: "" });
  const [directoryResult] = createResource(filters, getProviderDirectory);

  const searchSource = createMemo(() => {
    const value = filters().q.trim();
    if (value.length < 2) {
      return undefined;
    }

    return value;
  });
  const [searchResult] = createResource(searchSource, (q) => searchMarketplace({ q, limit: 8 }));

  const providers = createMemo(() => resultData(directoryResult())?.providers ?? []);
  const visibleProviders = createMemo(() =>
    filterProvidersByMedia(sortProviders(providers(), sortMode()), mediaMode())
  );
  const searchResults = createMemo(() => resultData(searchResult())?.results ?? []);
  const areaOptions = createMemo(() => serviceAreaOptions(providers(), serviceArea()));
  const appliedQuery = createMemo(() => filters().q);
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

  return (
    <section class="template-directory" aria-label="Provider directory">
      <div class="template-directory__intro">
        <div class="template-directory__intro-copy">
          <h1>Browse service providers</h1>
          <p>
            Search published profiles, compare service areas, and open the provider page for
            details, media, updates, and payment entry points.
          </p>
        </div>
        <div class="template-directory__stats" aria-label="Directory summary">
          <strong>{providers().length}</strong>
          <span>published profiles</span>
        </div>
      </div>

      <div class="template-shell">
        <ProviderDirectorySearch
          activeSection={activeSection()}
          mediaMode={mediaMode()}
          query={query()}
          resultLabel={directoryResult.loading ? "Refreshing results" : resultSummary(visibleProviders())}
          serviceArea={serviceArea()}
          serviceAreas={areaOptions()}
          sortMode={sortMode()}
          onApplyFilters={applyFilters}
          onMediaModeChange={setMediaMode}
          onQueryChange={setQuery}
          onSectionChange={setActiveSection}
          onServiceAreaChange={setServiceArea}
          onSortModeChange={setSortMode}
        />

        <ProviderSearchResults
          errorMessage={searchErrorMessage(searchResult())}
          loading={searchResult.loading}
          query={appliedQuery()}
          results={searchResults()}
          onOpenProvider={props.onOpenProvider}
        />

        <div class="template-directory-grid">
          {sectionVisible(activeSection(), "profiles") && (
            <ProviderListingGrid
              errorMessage={errorMessage()}
              loading={directoryResult.loading}
              providers={visibleProviders()}
              onOpenProvider={props.onOpenProvider}
            />
          )}
          <ProviderDirectorySections
            activeSection={activeSection()}
            onSectionChange={setActiveSection}
          />
        </div>
      </div>
    </section>
  );
}

function searchErrorMessage(result: Awaited<ReturnType<typeof searchMarketplace>> | undefined): string | null {
  if (result === undefined || result.ok) {
    return null;
  }

  return result.error.message;
}
