import { describe, expect, it } from "vitest";

import { pageFromPath, pathForPage } from "./navigation";
import type { LinkTokens } from "./shared/types";

const emptyTokens: LinkTokens = {
  resetToken: null,
  verificationToken: null
};

describe("navigation", () => {
  it("maps known paths to pages", () => {
    expect(pageFromPath("/", emptyTokens)).toBe("home");
    expect(pageFromPath("/join", emptyTokens)).toBe("join");
    expect(pageFromPath("/sign-in", emptyTokens)).toBe("signin");
    expect(pageFromPath("/verify-email", emptyTokens)).toBe("verify-email");
    expect(pageFromPath("/missing", emptyTokens)).toBe("not-found");
  });

  it("lets emailed link tokens select their workflows", () => {
    expect(pageFromPath("/anything", { resetToken: null, verificationToken: "token" })).toBe(
      "verify-email"
    );
    expect(pageFromPath("/anything", { resetToken: "token", verificationToken: null })).toBe(
      "recovery"
    );
  });

  it("maps page ids back to browser paths", () => {
    expect(pathForPage("join")).toBe("/join");
    expect(pathForPage("signin")).toBe("/sign-in");
    expect(pathForPage("admin-verification")).toBe("/admin");
  });
});
