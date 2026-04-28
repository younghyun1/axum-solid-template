import { describe, expect, it } from "vitest";

import { buildUrl, isApiEnvelope, requestApi } from "./client";
import type { HealthcheckResponse, JsonObject } from "./types";

describe("buildUrl", () => {
  it("attaches query values without mutating the path", () => {
    const url = buildUrl("/api/v1/auth/verify-user-email", "http://localhost:3000", {
      email_validation_token_id: "018f2f7a-7a39-7d89-a03f-b8b5e1d65021"
    });

    expect(url).toBe(
      "http://localhost:3000/api/v1/auth/verify-user-email?email_validation_token_id=018f2f7a-7a39-7d89-a03f-b8b5e1d65021"
    );
  });
});

describe("isApiEnvelope", () => {
  it("accepts the backend envelope shape", () => {
    const value: unknown = {
      success: true,
      data: { accepting_traffic: true },
      error: null,
      meta: {
        timestamp: "2026-04-28T10:35:12.123Z",
        processing_duration: "PT0S"
      }
    };

    expect(isApiEnvelope(value)).toBe(true);
  });
});

describe("requestApi", () => {
  it("sends bearer tokens and parses successful envelopes", async () => {
    const fetcher: typeof fetch = async (_input, init) => {
      const headers = new Headers(init?.headers);

      expect(headers.get("Authorization")).toBe("Bearer test-token");

      return new Response(
        JSON.stringify({
          success: true,
          data: { accepting_traffic: true },
          error: null,
          meta: {
            timestamp: "2026-04-28T10:35:12.123Z",
            processing_duration: "PT0S"
          }
        }),
        {
          headers: { "Content-Type": "application/json" },
          status: 200
        }
      );
    };

    const result = await requestApi<HealthcheckResponse, JsonObject>({
      baseUrl: "http://localhost:3000",
      fetcher,
      method: "GET",
      path: "/api/v1/healthcheck",
      token: "test-token"
    });

    expect(result.ok).toBe(true);
    if (result.ok) {
      expect(result.data?.accepting_traffic).toBe(true);
    }
  });

  it("normalizes backend error envelopes", async () => {
    const fetcher: typeof fetch = async () =>
      new Response(
        JSON.stringify({
          success: false,
          data: null,
          error: {
            error_code: 42,
            error_level: "WARN",
            error_message: "Rate limited",
            error_detail: "retry later"
          },
          meta: {
            timestamp: "2026-04-28T10:35:12.123Z",
            processing_duration: "PT0S"
          }
        }),
        {
          headers: { "Content-Type": "application/json" },
          status: 429
        }
      );

    const result = await requestApi<HealthcheckResponse>({
      baseUrl: "http://localhost:3000",
      fetcher,
      method: "GET",
      path: "/api/v1/healthcheck"
    });

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.kind).toBe("backend");
      expect(result.error.message).toBe("Rate limited: retry later");
      expect(result.error.backendError?.error_code).toBe(42);
    }
  });
});
