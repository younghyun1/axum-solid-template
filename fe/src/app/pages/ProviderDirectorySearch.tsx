import { For } from "solid-js";

import type { ReferenceSubdivisionResponse } from "../../api/types";
import type {
  DirectorySection,
  ProviderMediaMode,
  ProviderSortMode
} from "./providerDirectoryModel";
import {
  referenceSubdivisionCompositeCode,
  selectedSubdivisionLabel
} from "./providerDirectoryModel";

const SECTION_FILTERS: readonly { readonly label: string; readonly value: DirectorySection }[] = [
  { label: "All", value: "all" },
  { label: "Profiles", value: "profiles" },
  { label: "Live", value: "live" },
  { label: "Guides", value: "guides" },
  { label: "Events", value: "events" }
];

interface ProviderDirectorySearchProps {
  readonly activeSection: DirectorySection;
  readonly mediaMode: ProviderMediaMode;
  readonly query: string;
  readonly resultLabel: string;
  readonly subdivisionCode: string;
  readonly subdivisionErrorMessage: string | null;
  readonly subdivisionLoading: boolean;
  readonly subdivisions: readonly ReferenceSubdivisionResponse[];
  readonly sortMode: ProviderSortMode;
  readonly onApplyFilters: () => void;
  readonly onMediaModeChange: (value: ProviderMediaMode) => void;
  readonly onQueryChange: (value: string) => void;
  readonly onSectionChange: (value: DirectorySection) => void;
  readonly onSubdivisionCodeChange: (value: string) => void;
  readonly onSortModeChange: (value: ProviderSortMode) => void;
}

export function ProviderDirectorySearch(props: ProviderDirectorySearchProps) {
  return (
    <>
      <form
        class="template-directory-search"
        aria-label="Search providers"
        onSubmit={(event) => {
          event.preventDefault();
          props.onApplyFilters();
        }}
      >
        <label class="template-search-box">
          <span class="sr-only">Search providers</span>
          <SearchIcon />
          <input
            aria-label="Search providers"
            placeholder="Search by name, profile, or update"
            type="search"
            value={props.query}
            onInput={(event) => props.onQueryChange(event.currentTarget.value)}
          />
        </label>

        <label>
          <span class="sr-only">Subdivision</span>
          <select
            aria-label="Subdivision"
            value={props.subdivisionCode}
            onInput={(event) => props.onSubdivisionCodeChange(event.currentTarget.value)}
          >
            <option value="">
              {props.subdivisionLoading ? "Loading areas" : "All UK areas"}
            </option>
            {props.subdivisionCode.length > 0 &&
              !props.subdivisions.some(
                (subdivision) =>
                  referenceSubdivisionCompositeCode(subdivision) === props.subdivisionCode
              ) && (
                <option value={props.subdivisionCode} selected>
                  {selectedSubdivisionLabel(props.subdivisions, props.subdivisionCode)}
                </option>
              )}
            <For each={props.subdivisions}>
              {(subdivision) => (
                <option
                  value={referenceSubdivisionCompositeCode(subdivision)}
                  selected={referenceSubdivisionCompositeCode(subdivision) === props.subdivisionCode}
                >
                  {subdivision.subdivision_name}
                </option>
              )}
            </For>
          </select>
          {props.subdivisionErrorMessage !== null && (
            <span class="template-field-note">{props.subdivisionErrorMessage}</span>
          )}
        </label>

        <label>
          <span class="sr-only">Sort providers</span>
          <select
            aria-label="Sort providers"
            value={props.sortMode}
            onInput={(event) => props.onSortModeChange(event.currentTarget.value as ProviderSortMode)}
          >
            <option value="recommended">Recommended</option>
            <option value="name">Provider name</option>
            <option value="area">Location</option>
          </select>
        </label>

        <label>
          <span class="sr-only">Media state</span>
          <select
            aria-label="Media state"
            value={props.mediaMode}
            onInput={(event) => props.onMediaModeChange(event.currentTarget.value as ProviderMediaMode)}
          >
            <option value="all">All media</option>
            <option value="with-image">Image available</option>
            <option value="without-image">Media pending</option>
          </select>
        </label>

        <button class="template-primary-button" type="submit">
          Search providers
        </button>
      </form>

      <div class="template-toolbar">
        <div class="template-section-title">
          <h2>Provider directory</h2>
          <p>{props.resultLabel}</p>
        </div>
        <div class="template-filters" aria-label="Filter marketplace sections">
          <For each={SECTION_FILTERS}>
            {(filter) => (
              <button
                class="template-filter"
                type="button"
                aria-pressed={props.activeSection === filter.value ? "true" : "false"}
                onClick={() => props.onSectionChange(filter.value)}
              >
                {filter.label}
              </button>
            )}
          </For>
        </div>
      </div>
    </>
  );
}

function SearchIcon() {
  return (
    <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
      <path
        d="m21 21-4.35-4.35M10.5 18a7.5 7.5 0 1 1 0-15 7.5 7.5 0 0 1 0 15Z"
        stroke="currentColor"
        stroke-linecap="round"
        stroke-width="2"
      />
    </svg>
  );
}
