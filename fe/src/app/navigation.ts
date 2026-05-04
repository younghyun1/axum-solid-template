import type { LinkTokens, PageId } from "./shared/types";

export function pageFromPath(pathname: string, linkTokens: LinkTokens): PageId {
  if (linkTokens.verificationToken !== null) {
    return "verify-email";
  }
  if (linkTokens.resetToken !== null) {
    return "recovery";
  }

  switch (pathname) {
    case "/":
      return "home";
    case "/providers":
      return "providers";
    case "/join":
      return "join";
    case "/sign-in":
      return "signin";
    case "/account":
      return "account";
    case "/user":
      return "user-marketplace";
    case "/provider":
      return "provider-dashboard";
    case "/marketplace-admin":
      return "admin-marketplace";
    case "/recovery":
      return "recovery";
    case "/verify-email":
      return "verify-email";
    case "/admin":
      return "admin-verification";
    default:
      if (pathname.startsWith("/providers/")) {
        return "provider-detail";
      }
      return "not-found";
  }
}

export function pathForPage(page: PageId): string {
  switch (page) {
    case "home":
      return "/";
    case "providers":
      return "/providers";
    case "provider-detail":
      return "/providers";
    case "join":
      return "/join";
    case "signin":
      return "/sign-in";
    case "account":
      return "/account";
    case "user-marketplace":
      return "/user";
    case "provider-dashboard":
      return "/provider";
    case "admin-marketplace":
      return "/marketplace-admin";
    case "recovery":
      return "/recovery";
    case "verify-email":
      return "/verify-email";
    case "admin-verification":
      return "/admin";
    case "not-found":
      return "/not-found";
  }
}

export function pathForProvider(slug: string): string {
  return `/providers/${encodeURIComponent(slug)}`;
}

export function providerSlugFromPath(pathname: string): string | null {
  const prefix = "/providers/";
  if (!pathname.startsWith(prefix)) {
    return null;
  }

  const encoded = pathname.slice(prefix.length);
  if (encoded.length === 0) {
    return null;
  }

  try {
    return decodeURIComponent(encoded);
  } catch {
    return null;
  }
}
