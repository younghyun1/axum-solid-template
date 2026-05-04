export type PageId =
  | "home"
  | "join"
  | "signin"
  | "account"
  | "recovery"
  | "verify-email"
  | "admin-verification"
  | "not-found";

export type ThemeMode = "light" | "dark";
export type NoticeKind = "idle" | "success" | "error";

export interface Notice {
  readonly kind: NoticeKind;
  readonly text: string;
}

export interface LinkTokens {
  readonly resetToken: string | null;
  readonly verificationToken: string | null;
}

export const emptyNotice: Notice = {
  kind: "idle",
  text: ""
};
