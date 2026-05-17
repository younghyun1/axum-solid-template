export interface UkAreaLink {
  readonly label: string;
  readonly subdivisionCode: string;
}

export const UK_AREA_LINKS: readonly UkAreaLink[] = [
  { label: "London", subdivisionCode: "GB-LND" },
  { label: "Manchester", subdivisionCode: "GB-MAN" },
  { label: "Birmingham", subdivisionCode: "GB-BIR" },
  { label: "Leeds", subdivisionCode: "GB-LDS" },
  { label: "Edinburgh", subdivisionCode: "GB-EDH" },
  { label: "Glasgow", subdivisionCode: "GB-GLG" },
  { label: "Bristol", subdivisionCode: "GB-BST" },
  { label: "Belfast", subdivisionCode: "GB-BFS" }
];

export function knownUkAreaLabel(subdivisionCode: string): string | null {
  const normalizedCode = subdivisionCode.trim().toUpperCase();
  const match = UK_AREA_LINKS.find((area) => area.subdivisionCode === normalizedCode);
  return match?.label ?? null;
}
