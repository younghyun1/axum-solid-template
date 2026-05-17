import { describe, expect, it } from "vitest";

import type { ProviderDirectoryCardResponse } from "../../api/marketplaceTypes";
import {
  filterProvidersByMedia,
  providerInitials,
  resultSummary,
  sectionVisible,
  serviceAreaOptions,
  sortProviders
} from "./providerDirectoryModel";

describe("providerDirectoryModel", () => {
  it("sorts providers without mutating the source list", () => {
    const providers = [provider("zeta", "Zeta", "Remote"), provider("alpha", "Alpha", "Austin")];

    const sorted = sortProviders(providers, "name");

    expect(sorted.map((item) => item.display_name)).toEqual(["Alpha", "Zeta"]);
    expect(providers.map((item) => item.display_name)).toEqual(["Zeta", "Alpha"]);
  });

  it("filters providers by published media state", () => {
    const providers = [
      provider("media", "With Media", "Remote", "/image.jpg"),
      provider("empty", "Without Media", "Remote")
    ];

    expect(filterProvidersByMedia(providers, "with-image").map((item) => item.slug)).toEqual([
      "media"
    ]);
    expect(filterProvidersByMedia(providers, "without-image").map((item) => item.slug)).toEqual([
      "empty"
    ]);
  });

  it("keeps selected service area available when filtered results are empty", () => {
    const providers = [provider("alpha", "Alpha", "Austin")];

    expect(serviceAreaOptions(providers, "Remote")).toEqual(["Austin", "Remote"]);
  });

  it("builds stable initials and summaries", () => {
    expect(providerInitials("North Star Services")).toBe("NS");
    expect(providerInitials(" ")).toBe("SP");
    expect(resultSummary([provider("alpha", "Alpha", "Remote")])).toBe("Showing 1 profile");
  });

  it("checks section visibility for all and direct matches", () => {
    expect(sectionVisible("all", "events")).toBe(true);
    expect(sectionVisible("events", "events")).toBe(true);
    expect(sectionVisible("guides", "profiles")).toBe(false);
  });
});

function provider(
  slug: string,
  displayName: string,
  serviceArea: string,
  imageUrl: string | null = null
): ProviderDirectoryCardResponse {
  return {
    display_name: displayName,
    headline: `${displayName} profile`,
    primary_image:
      imageUrl === null
        ? null
        : {
            bucket: "public",
            byte_size: 100,
            created_at: "2026-05-17T00:00:00Z",
            height: 100,
            image_id: `${slug}-image`,
            image_type: "provider_profile",
            mime_type: "image/jpeg",
            object_key: `${slug}.jpg`,
            public_url: imageUrl,
            upload_status: "uploaded",
            uploaded_at: "2026-05-17T00:00:00Z",
            visibility: "public",
            width: 100
          },
    provider_profile_id: `${slug}-id`,
    service_area: serviceArea,
    slug
  };
}
