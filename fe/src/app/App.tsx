import { createEffect, createMemo, createResource, createSignal, Show } from "solid-js";
import { useLocation, useNavigate } from "@solidjs/router";

import {
  getCountries,
  getHealthcheck,
  getLanguages,
  logout,
  me
} from "../api/appApi";
import type { LoginResponse, MeResponse } from "../api/types";
import { initialTheme, profileFromSession, readLinkTokensFromSearch, resultData } from "./helpers";
import { pageFromPath, pathForPage } from "./navigation";
import { AccountPage } from "./pages/AccountPage";
import { AdminVerificationQuestionsPage } from "./pages/AdminVerificationQuestionsPage";
import { HomePage } from "./pages/HomePage";
import { JoinPage } from "./pages/JoinPage";
import { NotFoundPage } from "./pages/NotFoundPage";
import { RecoveryPage } from "./pages/RecoveryPage";
import { SignInPage } from "./pages/SignInPage";
import { VerifyEmailPage } from "./pages/VerifyEmailPage";
import type { PageId, ThemeMode } from "./shared/types";

export function App() {
  const location = useLocation();
  const navigate = useNavigate();
  const linkTokens = createMemo(() => readLinkTokensFromSearch(location.search));
  const activePage = createMemo(() => pageFromPath(location.pathname, linkTokens()));
  const [theme, setTheme] = createSignal<ThemeMode>(initialTheme());
  const [displayLanguage, setDisplayLanguage] = createSignal("en");
  const [token, setToken] = createSignal("");
  const [session, setSession] = createSignal<LoginResponse | null>(null);
  const [profile, setProfile] = createSignal<MeResponse | null>(null);
  const [menuOpen, setMenuOpen] = createSignal(false);

  const [countriesResult] = createResource(getCountries);
  const [languagesResult] = createResource(getLanguages);
  const [healthResult] = createResource(getHealthcheck);

  const countries = createMemo(() => resultData(countriesResult()) ?? []);
  const languages = createMemo(() => resultData(languagesResult()) ?? []);
  const healthOnline = createMemo(() => resultData(healthResult())?.accepting_traffic === true);
  const currentUser = createMemo(() => profile() ?? profileFromSession(session()));
  const isSignedIn = createMemo(() => token().trim().length > 0 && currentUser() !== null);
  const isAdmin = createMemo(() => {
    const user = currentUser();
    if (user === null) {
      return false;
    }

    return user.claims.role_type === "admin";
  });

  createEffect(() => {
    const selectedTheme = theme();
    document.documentElement.dataset["theme"] = selectedTheme;
    window.localStorage.setItem("preferred-theme", selectedTheme);
  });

  const toggleTheme = () => {
    setTheme((current) => (current === "light" ? "dark" : "light"));
  };

  const handleLogin = async (response: LoginResponse) => {
    setToken(response.access_token);
    setSession(response);
    setProfile(null);

    const profileResult = await me(response.access_token);
    if (profileResult.ok && profileResult.data !== null) {
      setProfile(profileResult.data);
    }

    goToPage("account");
  };

  const clearSession = async () => {
    if (token().trim().length > 0) {
      await logout(token());
    }

    setSession(null);
    setProfile(null);
    setToken("");
    setMenuOpen(false);
    goToPage("home");
  };

  const goToSwagger = () => {
    window.location.assign("/api/v1/swagger-ui/");
  };

  const goToPage = (page: PageId) => {
    navigate(pathForPage(page));
  };

  return (
    <div class="app-shell">
      <header class="top-bar">
        <button class="brand-button" type="button" onClick={() => goToPage("home")}>
          Rust-Solid-Template
        </button>

        <div class="top-actions">
          <button
            aria-label={theme() === "light" ? "Switch to dark theme" : "Switch to light theme"}
            aria-pressed={theme() === "dark" ? "true" : "false"}
            class="utility-button utility-button--icon"
            type="button"
            onClick={toggleTheme}
          >
            {theme() === "light" ? "🌙" : "☀️"}
          </button>
          <label class="select-control">
            <span class="sr-only">Language</span>
            <select
              value={displayLanguage()}
              onChange={(event) => setDisplayLanguage(event.currentTarget.value)}
            >
              <option value="en">English</option>
              <option value="ko">Korean</option>
              <option value="fr">French</option>
              <option value="de">German</option>
            </select>
          </label>

          <Show
            when={isSignedIn() && currentUser() !== null}
            fallback={
              <div class="guest-actions">
                <button class="secondary-button" type="button" onClick={() => goToPage("signin")}>
                  Sign in
                </button>
                <button class="primary-button" type="button" onClick={() => goToPage("join")}>
                  Create account
                </button>
              </div>
            }
          >
            <div class="user-menu">
              <Show when={isAdmin()}>
                <>
                  <button
                    class="secondary-button"
                    type="button"
                    onClick={() => goToPage("admin-verification")}
                  >
                    Admin Panel
                  </button>
                  <button class="secondary-button" type="button" onClick={goToSwagger}>
                    Swagger
                  </button>
                </>
              </Show>
              <span class="session-dot session-dot--in" aria-hidden="true" />
              <div class="user-summary">
                <span>{currentUser()?.user_info.user_name}</span>
                <small>{currentUser()?.user_info.user_email}</small>
              </div>
              <button
                class="avatar-button"
                type="button"
                aria-haspopup="menu"
                aria-expanded={menuOpen() ? "true" : "false"}
                onClick={() => setMenuOpen((open) => !open)}
              >
                {currentUser()?.user_info.user_name.slice(0, 1).toUpperCase()}
              </button>
              <Show when={menuOpen()}>
                <div class="profile-menu" role="menu">
                  <button
                    type="button"
                    role="menuitem"
                    onClick={() => {
                      goToPage("account");
                      setMenuOpen(false);
                    }}
                  >
                    Account
                  </button>
                  <button type="button" role="menuitem" onClick={clearSession}>
                    Sign out
                  </button>
                </div>
              </Show>
            </div>
          </Show>
        </div>
      </header>

      <main>
        <Show when={activePage() === "home"}>
          <HomePage
            isSignedIn={isSignedIn()}
            serviceOnline={healthOnline()}
            onCreateAccount={() => goToPage("join")}
            onSignIn={() => goToPage("signin")}
          />
        </Show>

        <Show when={activePage() === "join"}>
          <JoinPage
            countries={countries()}
            languages={languages()}
            onSignIn={() => goToPage("signin")}
          />
        </Show>

        <Show when={activePage() === "signin"}>
          <SignInPage
            onForgotPassword={() => goToPage("recovery")}
            onJoin={() => goToPage("join")}
            onLogin={handleLogin}
          />
        </Show>

        <Show when={activePage() === "account"}>
          <AccountPage
            countries={countries()}
            languages={languages()}
            profile={currentUser()}
            onSignIn={() => goToPage("signin")}
            onSignOut={clearSession}
          />
        </Show>

        <Show when={activePage() === "verify-email"}>
          <VerifyEmailPage
            isSignedIn={isSignedIn()}
            linkTokens={linkTokens()}
            token={token()}
            onHome={() => goToPage("home")}
            onProfileLoaded={(nextProfile) => setProfile(nextProfile)}
            onSignIn={() => goToPage("signin")}
          />
        </Show>

        <Show when={activePage() === "recovery"}>
          <RecoveryPage linkTokens={linkTokens()} onSignIn={() => goToPage("signin")} />
        </Show>

        <Show when={activePage() === "admin-verification"}>
          <AdminVerificationQuestionsPage
            isAdmin={isAdmin()}
            token={token()}
            onHome={() => goToPage("home")}
            onSignIn={() => goToPage("signin")}
          />
        </Show>

        <Show when={activePage() === "not-found"}>
          <NotFoundPage onHome={() => goToPage("home")} />
        </Show>
      </main>
    </div>
  );
}
