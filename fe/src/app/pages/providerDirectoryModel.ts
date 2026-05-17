import type { ProviderDirectoryCardResponse } from "../../api/marketplaceTypes";

export type ProviderSortMode = "recommended" | "name" | "area";
export type ProviderMediaMode = "all" | "with-image" | "without-image";
export type DirectorySection = "all" | "profiles" | "live" | "guides" | "events";

export interface DirectoryFilters {
  readonly q: string;
  readonly service_area: string;
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
      (left.service_area ?? "").localeCompare(right.service_area ?? "")
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

export function serviceAreaOptions(
  providers: readonly ProviderDirectoryCardResponse[],
  selectedArea: string
): readonly string[] {
  const areas = new Set<string>();
  for (const provider of providers) {
    const area = normalizedArea(provider.service_area);
    if (area !== null) {
      areas.add(area);
    }
  }

  const selected = normalizedArea(selectedArea);
  if (selected !== null) {
    areas.add(selected);
  }

  return [...areas].sort((left, right) => left.localeCompare(right));
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

function normalizedArea(value: string | null): string | null {
  if (value === null) {
    return null;
  }

  const trimmed = value.trim();
  if (trimmed.length === 0) {
    return null;
  }

  return trimmed;
}

function hasPublicImage(provider: ProviderDirectoryCardResponse): boolean {
  const image = provider.primary_image;
  if (image === null) {
    return false;
  }

  return image.public_url !== null;
}
