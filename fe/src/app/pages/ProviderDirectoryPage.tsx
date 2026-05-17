import { createEffect, createMemo, createResource, createSignal } from "solid-js";
import { useLocation, useNavigate } from "@solidjs/router";

import { getCountries, getCountrySubdivisions } from "../../api/appApi";
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
  sortProviders,
  ukCountryCode
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
  const location = useLocation();
  const navigate = useNavigate();
  const initialFilters = readDirectoryFilters(location.search);
  const [query, setQuery] = createSignal(initialFilters.q);
  const [subdivisionCode, setSubdivisionCode] = createSignal(initialFilters.subdivision_code);
  const [sortMode, setSortMode] = createSignal<ProviderSortMode>("recommended");
  const [mediaMode, setMediaMode] = createSignal<ProviderMediaMode>("all");
  const [activeSection, setActiveSection] = createSignal<DirectorySection>("all");
  const [filters, setFilters] = createSignal<DirectoryFilters>(initialFilters);
  const [directoryResult] = createResource(filters, getProviderDirectory);
  const [countriesResult] = createResource(getCountries);
  const countries = createMemo(() => resultData(countriesResult()) ?? []);
  const gbCountryCode = createMemo(() => ukCountryCode(countries()));
  const [subdivisionsResult] = createResource(gbCountryCode, async (countryCode) => {
    if (countryCode === null) {
      return undefined;
    }

    return getCountrySubdivisions(countryCode);
  });
  const subdivisions = createMemo(() => resultData(subdivisionsResult()) ?? []);

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
  const appliedQuery = createMemo(() => filters().q);
  const errorMessage = createMemo(() => {
    const result = directoryResult();
    if (result === undefined || result.ok) {
      return null;
    }

    return result.error.message;
  });
  const subdivisionErrorMessage = createMemo(() => {
    const result = subdivisionsResult();
    if (result === undefined || result.ok) {
      return null;
    }

    return result.error.message;
  });

  createEffect(() => {
    const nextFilters = readDirectoryFilters(location.search);
    if (nextFilters.q !== filters().q || nextFilters.subdivision_code !== filters().subdivision_code) {
      setQuery(nextFilters.q);
      setSubdivisionCode(nextFilters.subdivision_code);
      setFilters(nextFilters);
    }
  });

  const applyFilters = () => {
    const nextFilters = {
      q: query().trim(),
      subdivision_code: subdivisionCode().trim()
    };
    setFilters(nextFilters);
    navigate(`/providers${searchForFilters(nextFilters)}`);
  };

  return (
    <section class="template-directory" aria-label="Provider directory">
      <div class="template-shell">
        <ProviderDirectorySearch
          activeSection={activeSection()}
          mediaMode={mediaMode()}
          query={query()}
          resultLabel={directoryResult.loading ? "Refreshing results" : resultSummary(visibleProviders())}
          subdivisionCode={subdivisionCode()}
          subdivisionErrorMessage={subdivisionErrorMessage()}
          subdivisionLoading={subdivisionsResult.loading}
          subdivisions={subdivisions()}
          sortMode={sortMode()}
          onApplyFilters={applyFilters}
          onMediaModeChange={setMediaMode}
          onQueryChange={setQuery}
          onSectionChange={setActiveSection}
          onSubdivisionCodeChange={setSubdivisionCode}
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

function readDirectoryFilters(search: string): DirectoryFilters {
  const params = new URLSearchParams(search);
  return {
    q: params.get("q")?.trim() ?? "",
    subdivision_code: params.get("subdivision")?.trim() ?? ""
  };
}

function searchForFilters(filters: DirectoryFilters): string {
  const params = new URLSearchParams();
  if (filters.q.length > 0) {
    params.set("q", filters.q);
  }
  if (filters.subdivision_code.length > 0) {
    params.set("subdivision", filters.subdivision_code);
  }

  const encoded = params.toString();
  if (encoded.length === 0) {
    return "";
  }

  return `?${encoded}`;
}

function searchErrorMessage(result: Awaited<ReturnType<typeof searchMarketplace>> | undefined): string | null {
  if (result === undefined || result.ok) {
    return null;
  }

  return result.error.message;
}
