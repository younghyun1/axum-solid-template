import type {
  ApiCallResult,
  LoginResponse,
  MeResponse,
  ReferenceCountryResponse,
  ReferenceLanguageResponse
} from "../api/types";
import type { LinkTokens, ThemeMode } from "./shared/types";

export function initialTheme(): ThemeMode {
  const storedTheme = window.localStorage.getItem("preferred-theme");
  if (storedTheme === "light" || storedTheme === "dark") {
    return storedTheme;
  }

  if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
    return "dark";
  }

  return "light";
}

export function readLinkTokens(): LinkTokens {
  return readLinkTokensFromSearch(window.location.search);
}

export function readLinkTokensFromSearch(search: string): LinkTokens {
  const searchParams = new URLSearchParams(search);
  return {
    resetToken: searchParams.get("password_reset_token") ?? searchParams.get("token"),
    verificationToken: searchParams.get("email_validation_token_id")
  };
}

export function resultData<TData>(result: ApiCallResult<TData> | undefined): TData | null {
  if (result === undefined || !result.ok) {
    return null;
  }

  return result.data;
}

export function preferredCountry(
  countries: readonly ReferenceCountryResponse[]
): ReferenceCountryResponse | null {
  const usCountry = countries.find((country) => country.country_alpha2 === "US");
  if (usCountry !== undefined) {
    return usCountry;
  }

  const firstCountry = countries[0];
  if (firstCountry !== undefined) {
    return firstCountry;
  }

  return null;
}

export function findCountry(
  countries: readonly ReferenceCountryResponse[],
  countryCode: string
): ReferenceCountryResponse | null {
  const parsedCountryCode = parseInteger(countryCode);
  if (parsedCountryCode === null) {
    return null;
  }

  const country = countries.find((candidate) => candidate.country_code === parsedCountryCode);
  if (country !== undefined) {
    return country;
  }

  return null;
}

export function languagesWithPrimaryFirst(
  languages: readonly ReferenceLanguageResponse[],
  primaryLanguageCode: number | null
): readonly ReferenceLanguageResponse[] {
  if (primaryLanguageCode === null) {
    return languages;
  }

  const primaryLanguage = languages.find(
    (language) => language.language_code === primaryLanguageCode
  );
  if (primaryLanguage === undefined) {
    return languages;
  }

  return [
    primaryLanguage,
    ...languages.filter((language) => language.language_code !== primaryLanguageCode)
  ];
}

export function parseInteger(value: string): number | null {
  const trimmedValue = value.trim();
  if (trimmedValue.length === 0) {
    return null;
  }

  const parsed = Number.parseInt(trimmedValue, 10);
  if (!Number.isSafeInteger(parsed) || parsed.toString() !== trimmedValue) {
    return null;
  }

  return parsed;
}

export function parseOptionalInteger(value: string): number | null | "invalid" {
  const trimmedValue = value.trim();
  if (trimmedValue.length === 0) {
    return null;
  }

  const parsed = parseInteger(trimmedValue);
  if (parsed === null) {
    return "invalid";
  }

  return parsed;
}

export function profileFromSession(session: LoginResponse | null): MeResponse | null {
  if (session === null) {
    return null;
  }

  return {
    claims: session.claims,
    user_info: session.user_info
  };
}

export function countryLabel(
  countries: readonly ReferenceCountryResponse[],
  countryCode: number
): string {
  const country = countries.find((candidate) => candidate.country_code === countryCode);
  if (country === undefined) {
    return countryCode.toString();
  }

  return `${country.country_flag} ${country.country_name}`;
}

export function languageLabel(
  languages: readonly ReferenceLanguageResponse[],
  languageCode: number
): string {
  const language = languages.find((candidate) => candidate.language_code === languageCode);
  if (language === undefined) {
    return languageCode.toString();
  }

  return language.language_name;
}
