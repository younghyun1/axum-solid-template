import type {
  ProviderDirectoryCardResponse,
  ProviderSubdivisionResponse
} from "../../api/marketplaceTypes";
import type { ReferenceCountryResponse, ReferenceSubdivisionResponse } from "../../api/types";
import { knownUkAreaLabel } from "../shared/ukAreas";

export type ProviderSortMode = "recommended" | "name" | "area";
export type ProviderMediaMode = "all" | "with-image" | "without-image";
export type DirectorySection = "all" | "profiles" | "live" | "guides" | "events";

export interface DirectoryFilters {
  readonly q: string;
  readonly subdivision_code: string;
}

export function sortProviders(
  providers: readonly ProviderDirectoryCardResponse[],
  sortMode: ProviderSortMode
): readonly ProviderDirectoryCardResponse[] {
  const sortedProviders = [...providers];
  if (sortMode === "name") {
    return sortedProviders.sort((left, right) => left.display_name.localeCompare(right.display_name));
  }

  if (sortMode === "area") {
    return sortedProviders.sort((left, right) =>
      subdivisionDisplayName(left.subdivision).localeCompare(subdivisionDisplayName(right.subdivision))
    );
  }

  return sortedProviders;
}

export function filterProvidersByMedia(
  providers: readonly ProviderDirectoryCardResponse[],
  mediaMode: ProviderMediaMode
): readonly ProviderDirectoryCardResponse[] {
  if (mediaMode === "with-image") {
    return providers.filter(hasPublicImage);
  }

  if (mediaMode === "without-image") {
    return providers.filter((provider) => !hasPublicImage(provider));
  }

  return providers;
}

export function providerInitials(displayName: string): string {
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

export function resultSummary(providers: readonly ProviderDirectoryCardResponse[]): string {
  if (providers.length === 1) {
    return "Showing 1 profile";
  }

  return `Showing ${providers.length} profiles`;
}

export function sectionVisible(activeSection: DirectorySection, section: DirectorySection): boolean {
  return activeSection === "all" || activeSection === section;
}

export function compactHeadline(provider: ProviderDirectoryCardResponse): string {
  return provider.headline ?? "Published marketplace profile";
}

export function providerLocationLabel(
  subdivision: ProviderSubdivisionResponse | null,
  fallback = "Location not listed"
): string {
  const label = subdivisionDisplayName(subdivision);
  if (label.length === 0) {
    return fallback;
  }

  return label;
}

export function subdivisionDisplayName(
  subdivision: ProviderSubdivisionResponse | null
): string {
  return subdivision?.subdivision_name ?? "";
}

export function referenceSubdivisionCompositeCode(
  subdivision: ReferenceSubdivisionResponse
): string {
  return `GB-${subdivision.subdivision_code}`;
}

export function selectedSubdivisionLabel(
  subdivisions: readonly ReferenceSubdivisionResponse[],
  selectedCompositeCode: string
): string {
  const selected = subdivisions.find(
    (subdivision) => referenceSubdivisionCompositeCode(subdivision) === selectedCompositeCode
  );
  if (selected === undefined) {
    return knownUkAreaLabel(selectedCompositeCode) ?? "Selected UK area";
  }

  return selected.subdivision_name;
}

export function ukCountryCode(countries: readonly ReferenceCountryResponse[]): number | null {
  const country = countries.find((candidate) => candidate.country_alpha2 === "GB");
  if (country === undefined) {
    return null;
  }

  return country.country_code;
}

function hasPublicImage(provider: ProviderDirectoryCardResponse): boolean {
  const image = provider.primary_image;
  if (image === null) {
    return false;
  }

  return image.public_url !== null;
}
