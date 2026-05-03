import { createEffect, createMemo, createResource, createSignal, Show } from "solid-js";

import {
  getCountries,
  getHealthcheck,
  getLanguages,
  logout,
  me
} from "../api/appApi";
import type { LoginResponse, MeResponse } from "../api/types";
import { initialTheme, profileFromSession, readLinkTokens, resultData } from "./helpers";
import { AccountPage } from "./pages/AccountPage";
import { AdminVerificationQuestionsPage } from "./pages/AdminVerificationQuestionsPage";
import { HomePage } from "./pages/HomePage";
import { JoinPage } from "./pages/JoinPage";
import { RecoveryPage } from "./pages/RecoveryPage";
import { SignInPage } from "./pages/SignInPage";
import { VerifyEmailPage } from "./pages/VerifyEmailPage";
import type { PageId, ThemeMode } from "./shared/types";

export function App() {
  const linkTokens = readLinkTokens();
  const [activePage, setActivePage] = createSignal<PageId>(
    linkTokens.verificationToken !== null
      ? "verify-email"
      : linkTokens.resetToken !== null
        ? "recovery"
        : "home"
  );
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

    setActivePage("account");
  };

  const clearSession = async () => {
    if (token().trim().length > 0) {
      await logout(token());
    }

    setSession(null);
    setProfile(null);
    setToken("");
    setMenuOpen(false);
    setActivePage("home");
  };

  const goToSwagger = () => {
    window.location.assign("/api/v1/swagger-ui/");
  };

  return (
    <div class="app-shell">
      <header class="top-bar">
        <button class="brand-button" type="button" onClick={() => setActivePage("home")}>
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
                <button class="secondary-button" type="button" onClick={() => setActivePage("signin")}>
                  Sign in
                </button>
                <button class="primary-button" type="button" onClick={() => setActivePage("join")}>
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
                    onClick={() => setActivePage("admin-verification")}
                  >
                    Verification challenges
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
                      setActivePage("account");
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
            onCreateAccount={() => setActivePage("join")}
            onSignIn={() => setActivePage("signin")}
          />
        </Show>

        <Show when={activePage() === "join"}>
          <JoinPage
            countries={countries()}
            languages={languages()}
            onSignedUp={() => setActivePage("signin")}
            onSignIn={() => setActivePage("signin")}
          />
        </Show>

        <Show when={activePage() === "signin"}>
          <SignInPage
            onForgotPassword={() => setActivePage("recovery")}
            onJoin={() => setActivePage("join")}
            onLogin={handleLogin}
          />
        </Show>

        <Show when={activePage() === "account"}>
          <AccountPage
            countries={countries()}
            languages={languages()}
            profile={currentUser()}
            onSignIn={() => setActivePage("signin")}
            onSignOut={clearSession}
          />
        </Show>

        <Show when={activePage() === "verify-email"}>
          <VerifyEmailPage
            isSignedIn={isSignedIn()}
            linkTokens={linkTokens}
            token={token()}
            onHome={() => setActivePage("home")}
            onProfileLoaded={(nextProfile) => setProfile(nextProfile)}
            onSignIn={() => setActivePage("signin")}
          />
        </Show>

        <Show when={activePage() === "recovery"}>
          <RecoveryPage linkTokens={linkTokens} onSignIn={() => setActivePage("signin")} />
        </Show>

        <Show when={activePage() === "admin-verification"}>
          <AdminVerificationQuestionsPage
            isAdmin={isAdmin()}
            token={token()}
            onHome={() => setActivePage("home")}
            onSignIn={() => setActivePage("signin")}
          />
        </Show>
      </main>
    </div>
  );
}
