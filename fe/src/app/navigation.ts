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
    case "/join":
      return "join";
    case "/sign-in":
      return "signin";
    case "/account":
      return "account";
    case "/recovery":
      return "recovery";
    case "/verify-email":
      return "verify-email";
    case "/admin":
      return "admin-verification";
    default:
      return "not-found";
  }
}

export function pathForPage(page: PageId): string {
  switch (page) {
    case "home":
      return "/";
    case "join":
      return "/join";
    case "signin":
      return "/sign-in";
    case "account":
      return "/account";
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
