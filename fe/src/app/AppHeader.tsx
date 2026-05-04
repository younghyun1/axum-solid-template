import { Show } from "solid-js";

import type { MeResponse } from "../api/types";
import type { PageId, ThemeMode } from "./shared/types";

interface AppHeaderProps {
  readonly theme: ThemeMode;
  readonly displayLanguage: string;
  readonly currentUser: MeResponse | null;
  readonly isSignedIn: boolean;
  readonly isAdmin: boolean;
  readonly isProvider: boolean;
  readonly canModerate: boolean;
  readonly menuOpen: boolean;
  readonly onThemeToggle: () => void;
  readonly onLanguageChange: (value: string) => void;
  readonly onPage: (page: PageId) => void;
  readonly onSwagger: () => void;
  readonly onToggleMenu: () => void;
  readonly onCloseMenu: () => void;
  readonly onSignOut: () => void;
}

export function AppHeader(props: AppHeaderProps) {
  return (
    <header class="top-bar">
      <button class="brand-button" type="button" onClick={() => props.onPage("home")}>
        Rust-Solid-Template
      </button>

      <nav class="primary-nav" aria-label="Marketplace">
        <button type="button" onClick={() => props.onPage("providers")}>
          Providers
        </button>
        <Show when={props.isSignedIn}>
          <button type="button" onClick={() => props.onPage("user-marketplace")}>
            User
          </button>
        </Show>
        <Show when={props.isProvider}>
          <button type="button" onClick={() => props.onPage("provider-dashboard")}>
            Provider
          </button>
        </Show>
        <Show when={props.canModerate}>
          <button type="button" onClick={() => props.onPage("admin-marketplace")}>
            Admin
          </button>
        </Show>
      </nav>

      <div class="top-actions">
        <button
          aria-label={props.theme === "light" ? "Switch to dark theme" : "Switch to light theme"}
          aria-pressed={props.theme === "dark" ? "true" : "false"}
          class="utility-button utility-button--icon"
          type="button"
          onClick={props.onThemeToggle}
        >
          {props.theme === "light" ? "🌙" : "☀️"}
        </button>
        <label class="select-control">
          <span class="sr-only">Language</span>
          <select
            value={props.displayLanguage}
            onChange={(event) => props.onLanguageChange(event.currentTarget.value)}
          >
            <option value="en">English</option>
            <option value="ko">Korean</option>
            <option value="fr">French</option>
            <option value="de">German</option>
          </select>
        </label>

        <Show
          when={props.isSignedIn && props.currentUser !== null}
          fallback={
            <div class="guest-actions">
              <button class="secondary-button" type="button" onClick={() => props.onPage("signin")}>
                Sign in
              </button>
              <button class="primary-button" type="button" onClick={() => props.onPage("join")}>
                Create account
              </button>
            </div>
          }
        >
          <div class="user-menu">
            <Show when={props.isAdmin}>
              <>
                <button
                  class="secondary-button"
                  type="button"
                  onClick={() => props.onPage("admin-verification")}
                >
                  Admin Panel
                </button>
                <button class="secondary-button" type="button" onClick={props.onSwagger}>
                  Swagger
                </button>
              </>
            </Show>
            <span class="session-dot session-dot--in" aria-hidden="true" />
            <div class="user-summary">
              <span>{props.currentUser?.user_info.user_name}</span>
              <small>{props.currentUser?.user_info.user_email}</small>
            </div>
            <button
              class="avatar-button"
              type="button"
              aria-haspopup="menu"
              aria-expanded={props.menuOpen ? "true" : "false"}
              onClick={props.onToggleMenu}
            >
              {props.currentUser?.user_info.user_name.slice(0, 1).toUpperCase()}
            </button>
            <Show when={props.menuOpen}>
              <div class="profile-menu" role="menu">
                <button
                  type="button"
                  role="menuitem"
                  onClick={() => {
                    props.onPage("account");
                    props.onCloseMenu();
                  }}
                >
                  Account
                </button>
                <button type="button" role="menuitem" onClick={props.onSignOut}>
                  Sign out
                </button>
              </div>
            </Show>
          </div>
        </Show>
      </div>
    </header>
  );
}
