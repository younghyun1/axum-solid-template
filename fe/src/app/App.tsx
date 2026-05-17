import { createEffect, createMemo, createResource, createSignal, Show } from "solid-js";
import { useLocation, useNavigate } from "@solidjs/router";

import {
  getCountries,
  getHealthcheck,
  getLanguages,
  logout,
  refreshSession
} from "../api/appApi";
import type { LoginResponse, MeResponse } from "../api/types";
import { AppHeader } from "./AppHeader";
import { initialTheme, profileFromSession, readLinkTokensFromSearch, resultData } from "./helpers";
import { pageFromPath, pathForPage, pathForProvider, providerSlugFromPath } from "./navigation";
import { AccountPage } from "./pages/AccountPage";
import { AdminMarketplacePage } from "./pages/AdminMarketplacePage";
import { AdminVerificationQuestionsPage } from "./pages/AdminVerificationQuestionsPage";
import { HomePage } from "./pages/HomePage";
import { JoinPage } from "./pages/JoinPage";
import { NotFoundPage } from "./pages/NotFoundPage";
import { ProviderDashboardPage } from "./pages/ProviderDashboardPage";
import { ProviderDetailPage } from "./pages/ProviderDetailPage";
import { ProviderDirectoryPage } from "./pages/ProviderDirectoryPage";
import { RecoveryPage } from "./pages/RecoveryPage";
import { SignInPage } from "./pages/SignInPage";
import { UserMarketplacePage } from "./pages/UserMarketplacePage";
import { VerifyEmailPage } from "./pages/VerifyEmailPage";
import type { PageId, ThemeMode } from "./shared/types";

export function App() {
  const location = useLocation();
  const navigate = useNavigate();
  const linkTokens = createMemo(() => readLinkTokensFromSearch(location.search));
  const activePage = createMemo(() => pageFromPath(location.pathname, linkTokens()));
  const [theme, setTheme] = createSignal<ThemeMode>(initialTheme());
  const [displayLanguage, setDisplayLanguage] = createSignal("en");
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
  const isSignedIn = createMemo(() => currentUser() !== null);
  const isAdmin = createMemo(() => {
    const user = currentUser();
    if (user === null) {
      return false;
    }

    return user.claims.role_type === "admin";
  });
  const isProvider = createMemo(() => {
    const user = currentUser();
    if (user === null) {
      return false;
    }

    return user.claims.role_type === "service_provider" || user.claims.role_type === "admin";
  });
  const canModerate = createMemo(() => {
    const user = currentUser();
    if (user === null) {
      return false;
    }

    return user.claims.role_type === "admin" || user.claims.role_type === "moderator";
  });

  const restoreSession = async () => {
    const result = await refreshSession();
    if (!result.ok || result.data === null) {
      return;
    }

    setSession(result.data);
    setProfile({ claims: result.data.claims, user_info: result.data.user_info });
  };

  createEffect(() => {
    const selectedTheme = theme();
    document.documentElement.dataset["theme"] = selectedTheme;
    window.localStorage.setItem("preferred-theme", selectedTheme);
  });

  createEffect(() => {
    void restoreSession();
  });

  const toggleTheme = () => {
    setTheme((current) => (current === "light" ? "dark" : "light"));
  };

  const handleLogin = async (response: LoginResponse) => {
    setSession(response);
    setProfile({ claims: response.claims, user_info: response.user_info });

    goToPage("account");
  };

  const dropSession = (nextPage: PageId) => {
    setSession(null);
    setProfile(null);
    setMenuOpen(false);
    goToPage(nextPage);
  };

  const clearSession = async () => {
    await logout();
    dropSession("home");
  };

  const goToSwagger = () => {
    window.location.assign("/api/v1/swagger-ui/");
  };

  const goToPage = (page: PageId) => {
    navigate(pathForPage(page));
  };

  const goToProviderSubdivision = (subdivisionCode: string) => {
    navigate(`/providers?subdivision=${encodeURIComponent(subdivisionCode)}`);
  };

  return (
    <div class="app-shell">
      <AppHeader
        theme={theme()}
        displayLanguage={displayLanguage()}
        currentUser={currentUser()}
        isSignedIn={isSignedIn()}
        isAdmin={isAdmin()}
        isProvider={isProvider()}
        canModerate={canModerate()}
        menuOpen={menuOpen()}
        onThemeToggle={toggleTheme}
        onLanguageChange={setDisplayLanguage}
        onPage={goToPage}
        onSubdivisionSelect={goToProviderSubdivision}
        onSwagger={goToSwagger}
        onToggleMenu={() => setMenuOpen((open) => !open)}
        onCloseMenu={() => setMenuOpen(false)}
        onSignOut={clearSession}
      />

      <main class={activePage() === "home" || activePage() === "providers" ? "app-main app-main--full" : "app-main"}>
        <Show when={activePage() === "home"}>
          <>
            <HomePage
              isSignedIn={isSignedIn()}
              serviceOnline={healthOnline()}
              onBrowseProviders={() => goToPage("providers")}
              onCreateAccount={() => goToPage("join")}
              onSignIn={() => goToPage("signin")}
            />
            <ProviderDirectoryPage onOpenProvider={(slug) => navigate(pathForProvider(slug))} />
          </>
        </Show>

        <Show when={activePage() === "providers"}>
          <ProviderDirectoryPage onOpenProvider={(slug) => navigate(pathForProvider(slug))} />
        </Show>

        <Show when={activePage() === "provider-detail"}>
          <ProviderDetailPage
            slug={providerSlugFromPath(location.pathname)}
            onBack={() => goToPage("providers")}
            onOpenPayments={() => goToPage("user-marketplace")}
          />
        </Show>

        <Show when={activePage() === "user-marketplace"}>
          <UserMarketplacePage profile={currentUser()} onSignIn={() => goToPage("signin")} />
        </Show>

        <Show when={activePage() === "provider-dashboard"}>
          <ProviderDashboardPage profile={currentUser()} />
        </Show>

        <Show when={activePage() === "admin-marketplace"}>
          <AdminMarketplacePage profile={currentUser()} />
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
            onDatabaseReset={() => dropSession("signin")}
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
