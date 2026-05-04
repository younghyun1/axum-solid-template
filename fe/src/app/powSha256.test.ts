import { describe, expect, it } from "vitest";

import { sha256 } from "./powSha256";

const encoder = new TextEncoder();

describe("sha256", () => {
  it("matches known SHA-256 vectors", () => {
    expect(hex(sha256(encoder.encode("")))).toBe(
      "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    expect(hex(sha256(encoder.encode("abc")))).toBe(
      "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
  });
});

function hex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}
