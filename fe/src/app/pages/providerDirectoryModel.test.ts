import { describe, expect, it } from "vitest";

import type { ProviderDirectoryCardResponse } from "../../api/marketplaceTypes";
import {
  filterProvidersByMedia,
  providerInitials,
  providerLocationLabel,
  referenceSubdivisionCompositeCode,
  resultSummary,
  selectedSubdivisionLabel,
  sectionVisible,
  sortProviders,
  ukCountryCode
} from "./providerDirectoryModel";

describe("providerDirectoryModel", () => {
  it("sorts providers without mutating the source list", () => {
    const providers = [
      provider("zeta", "Zeta", subdivision("ZZZ", "Zeta Area")),
      provider("alpha", "Alpha", subdivision("AAA", "Alpha Area"))
    ];

    const sorted = sortProviders(providers, "name");

    expect(sorted.map((item) => item.display_name)).toEqual(["Alpha", "Zeta"]);
    expect(providers.map((item) => item.display_name)).toEqual(["Zeta", "Alpha"]);
  });

  it("filters providers by published media state", () => {
    const providers = [
      provider("media", "With Media", subdivision("REM", "Remote"), "/image.jpg"),
      provider("empty", "Without Media", subdivision("REM", "Remote"))
    ];

    expect(filterProvidersByMedia(providers, "with-image").map((item) => item.slug)).toEqual([
      "media"
    ]);
    expect(filterProvidersByMedia(providers, "without-image").map((item) => item.slug)).toEqual([
      "empty"
    ]);
  });

  it("sorts providers by subdivision name", () => {
    const providers = [
      provider("zeta", "Zeta", subdivision("ZZZ", "Zeta Area")),
      provider("alpha", "Alpha", subdivision("AAA", "Alpha Area"))
    ];

    expect(sortProviders(providers, "area").map((item) => item.slug)).toEqual(["alpha", "zeta"]);
  });

  it("builds stable initials and summaries", () => {
    expect(providerInitials("North Star Services")).toBe("NS");
    expect(providerInitials(" ")).toBe("SP");
    expect(resultSummary([provider("alpha", "Alpha", subdivision("REM", "Remote"))])).toBe(
      "Showing 1 profile"
    );
  });

  it("checks section visibility for all and direct matches", () => {
    expect(sectionVisible("all", "events")).toBe(true);
    expect(sectionVisible("events", "events")).toBe(true);
    expect(sectionVisible("guides", "profiles")).toBe(false);
  });

  it("formats subdivision labels and composite codes", () => {
    const remote = subdivision("REM", "Remote");

    expect(providerLocationLabel(remote)).toBe("Remote");
    expect(providerLocationLabel(null)).toBe("Location not listed");
    expect(referenceSubdivisionCompositeCode(referenceSubdivision("LND", "London"))).toBe("GB-LND");
    expect(selectedSubdivisionLabel([referenceSubdivision("LND", "London")], "GB-LND")).toBe(
      "London"
    );
    expect(selectedSubdivisionLabel([], "GB-LND")).toBe("London");
    expect(selectedSubdivisionLabel([], "GB-BST")).toBe("Bristol");
    expect(selectedSubdivisionLabel([], "GB-XYZ")).toBe("Selected UK area");
    expect(ukCountryCode([{ ...country(), country_alpha2: "US" }, country()])).toBe(826);
  });
});

function provider(
  slug: string,
  displayName: string,
  providerSubdivision: ProviderDirectoryCardResponse["subdivision"],
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
    subdivision: providerSubdivision,
    slug
  };
}

function subdivision(subdivisionCode: string, subdivisionName: string) {
  return {
    country_alpha2: "GB",
    country_code: 826,
    subdivision_code: subdivisionCode,
    subdivision_id: 1,
    subdivision_name: subdivisionName,
    subdivision_type: "city"
  };
}

function referenceSubdivision(subdivisionCode: string, subdivisionName: string) {
  return {
    country_flag: "🇬🇧",
    country_code: 826,
    subdivision_code: subdivisionCode,
    subdivision_id: 1,
    subdivision_name: subdivisionName,
    subdivision_type: "city"
  };
}

function country() {
  return {
    country_alpha2: "GB",
    country_alpha3: "GBR",
    country_code: 826,
    country_currency: 826,
    country_flag: "🇬🇧",
    country_name: "United Kingdom",
    country_primary_language: 1,
    is_country: true,
    phone_prefix: "+44"
  };
}
