import { createEffect, createMemo, createResource, createSignal, For, Show } from "solid-js";

import {
  checkIfUserExists,
  getCountries,
  getCountrySubdivisions,
  getHealthcheck,
  getLanguages,
  login,
  logout,
  me,
  requestPasswordReset,
  resetPassword,
  signup,
  verifyEmail
} from "../api/appApi";
import type {
  ApiCallResult,
  LoginResponse,
  MeResponse,
  ReferenceCountryResponse,
  ReferenceLanguageResponse,
  ReferenceSubdivisionResponse,
  SignupRequest
} from "../api/types";

type PageId = "home" | "join" | "signin" | "account" | "recovery";
type ThemeMode = "light" | "dark";
type NoticeKind = "idle" | "success" | "error";

interface PageDefinition {
  readonly id: PageId;
  readonly label: string;
}

interface Notice {
  readonly kind: NoticeKind;
  readonly text: string;
}

interface LinkTokens {
  readonly resetToken: string | null;
  readonly verificationToken: string | null;
}

const pages: readonly PageDefinition[] = [
  { id: "home", label: "Home" },
  { id: "join", label: "Create account" },
  { id: "signin", label: "Sign in" },
  { id: "account", label: "Account" },
  { id: "recovery", label: "Recovery" }
];

const emptyNotice: Notice = {
  kind: "idle",
  text: ""
};

export function App() {
  const linkTokens = readLinkTokens();
  const [activePage, setActivePage] = createSignal<PageId>(
    linkTokens.resetToken !== null || linkTokens.verificationToken !== null ? "recovery" : "home"
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

  return (
    <div class="app-shell">
      <header class="top-bar">
        <button class="brand-button" type="button" onClick={() => setActivePage("home")}>
          Home
        </button>

        <nav class="page-nav" aria-label="Primary navigation">
          <For each={pages}>
            {(page) => (
              <button
                class="nav-button"
                classList={{ "nav-button--active": activePage() === page.id }}
                type="button"
                onClick={() => setActivePage(page.id)}
              >
                {page.label}
              </button>
            )}
          </For>
        </nav>

        <div class="top-actions">
          <button
            aria-label="Toggle color theme"
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
                <span class="session-dot session-dot--out" aria-hidden="true" />
                <button class="secondary-button" type="button" onClick={() => setActivePage("signin")}>
                  Sign in
                </button>
              </div>
            }
          >
            <div class="user-menu">
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

        <Show when={activePage() === "recovery"}>
          <RecoveryPage linkTokens={linkTokens} onSignIn={() => setActivePage("signin")} />
        </Show>
      </main>
    </div>
  );
}

interface HomePageProps {
  readonly isSignedIn: boolean;
  readonly serviceOnline: boolean;
  readonly onCreateAccount: () => void;
  readonly onSignIn: () => void;
}

function HomePage(props: HomePageProps) {
  return (
    <section class="page-view landing-layout">
      <div class="landing-copy">
        <p class="eyebrow">Account portal</p>
        <h1>Access your account without the backend getting in the way.</h1>
        <p class="hero-text">
          Sign in, create an account, manage your profile, and recover access from one clean
          customer-facing flow.
        </p>
        <div class="hero-actions">
          <button class="primary-button" type="button" onClick={props.onCreateAccount}>
            Create account
          </button>
          <button class="secondary-button" type="button" onClick={props.onSignIn}>
            Sign in
          </button>
        </div>
      </div>

      <aside class="portal-panel">
        <div class="portal-panel__header">
          <span class={props.serviceOnline ? "status-light status-light--ok" : "status-light"} />
          <div>
            <p class="eyebrow">Service</p>
            <h2>{props.serviceOnline ? "Available" : "Checking status"}</h2>
          </div>
        </div>
        <dl class="summary-list">
          <div>
            <dt>Session</dt>
            <dd>{props.isSignedIn ? "Signed in" : "Guest"}</dd>
          </div>
          <div>
            <dt>Recovery</dt>
            <dd>Email link</dd>
          </div>
          <div>
            <dt>Profile</dt>
            <dd>Protected</dd>
          </div>
        </dl>
      </aside>
    </section>
  );
}

interface JoinPageProps {
  readonly countries: readonly ReferenceCountryResponse[];
  readonly languages: readonly ReferenceLanguageResponse[];
  readonly onSignedUp: () => void;
  readonly onSignIn: () => void;
}

function JoinPage(props: JoinPageProps) {
  const [userName, setUserName] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");
  const [countryCode, setCountryCode] = createSignal("");
  const [languageCode, setLanguageCode] = createSignal("");
  const [subdivisionId, setSubdivisionId] = createSignal("");
  const [subdivisions, setSubdivisions] = createSignal<readonly ReferenceSubdivisionResponse[]>([]);
  const [notice, setNotice] = createSignal<Notice>(emptyNotice);
  const [running, setRunning] = createSignal(false);
  let subdivisionRequestSequence = 0;

  const selectedCountry = createMemo(() => findCountry(props.countries, countryCode()));
  const orderedLanguages = createMemo(() =>
    languagesWithPrimaryFirst(props.languages, selectedCountry()?.country_primary_language ?? null)
  );
  const passwordsMismatch = createMemo(
    () => password().length > 0 && confirmPassword().length > 0 && password() !== confirmPassword()
  );

  createEffect(() => {
    if (countryCode().length > 0) {
      return;
    }

    const country = preferredCountry(props.countries);
    if (country === null) {
      return;
    }

    setCountryCode(country.country_code.toString());
    setLanguageCode(country.country_primary_language.toString());
  });

  createEffect(() => {
    const country = selectedCountry();
    if (country === null) {
      setSubdivisions([]);
      setSubdivisionId("");
      return;
    }

    setLanguageCode(country.country_primary_language.toString());
    setSubdivisionId("");
    subdivisionRequestSequence += 1;
    const requestSequence = subdivisionRequestSequence;
    void getCountrySubdivisions(country.country_code).then((result) => {
      if (requestSequence !== subdivisionRequestSequence) {
        return;
      }

      setSubdivisions(result.ok ? result.data ?? [] : []);
    });
  });

  const submit = async (event: SubmitEvent) => {
    event.preventDefault();
    const userCountry = parseInteger(countryCode());
    const userLanguage = parseInteger(languageCode());
    const userSubdivision = parseOptionalInteger(subdivisionId());

    if (userCountry === null || userLanguage === null) {
      setNotice({ kind: "error", text: "Choose a country and language." });
      return;
    }

    if (userSubdivision === "invalid") {
      setNotice({ kind: "error", text: "Choose a valid subdivision." });
      return;
    }

    if (passwordsMismatch()) {
      setNotice({ kind: "error", text: "Passwords do not match." });
      return;
    }

    setRunning(true);
    const emailCheck = await checkIfUserExists({ user_email: email().trim() });
    if (emailCheck.ok && emailCheck.data?.email_exists === true) {
      setRunning(false);
      setNotice({ kind: "error", text: "That email is already registered." });
      return;
    }

    const body: SignupRequest = {
      user_country: userCountry,
      user_email: email().trim(),
      user_language: userLanguage,
      user_name: userName().trim(),
      user_password: password(),
      user_subdivision: userSubdivision
    };
    const result = await signup(body);
    setRunning(false);

    if (!result.ok) {
      setNotice({ kind: "error", text: result.error.message });
      return;
    }

    setNotice({
      kind: "success",
      text: "Account created. Check your email to verify your address."
    });
    props.onSignedUp();
  };

  return (
    <section class="page-view auth-page">
      <div class="auth-card">
        <p class="eyebrow">New account</p>
        <h1>Create your account</h1>
        <form class="flow-form" onSubmit={submit}>
          <input
            autocomplete="username"
            placeholder="Username"
            required
            value={userName()}
            onInput={(event) => setUserName(event.currentTarget.value)}
          />
          <input
            autocomplete="email"
            placeholder="Email"
            required
            type="email"
            value={email()}
            onInput={(event) => setEmail(event.currentTarget.value)}
          />
          <input
            autocomplete="new-password"
            placeholder="Password"
            required
            type="password"
            value={password()}
            onInput={(event) => setPassword(event.currentTarget.value)}
          />
          <input
            autocomplete="new-password"
            aria-invalid={passwordsMismatch() ? "true" : "false"}
            placeholder="Re-enter password"
            required
            type="password"
            value={confirmPassword()}
            onInput={(event) => setConfirmPassword(event.currentTarget.value)}
          />
          <Show when={passwordsMismatch()}>
            <p class="field-note field-note--error">Passwords do not match.</p>
          </Show>
          <select
            required
            value={countryCode()}
            onChange={(event) => setCountryCode(event.currentTarget.value)}
          >
            <option value="">Select country</option>
            <For each={props.countries}>
              {(country) => (
                <option value={country.country_code}>
                  {country.country_flag} {country.country_name}
                </option>
              )}
            </For>
          </select>
          <select
            required
            value={languageCode()}
            onChange={(event) => setLanguageCode(event.currentTarget.value)}
          >
            <option value="">Select language</option>
            <For each={orderedLanguages()}>
              {(language) => <option value={language.language_code}>{language.language_name}</option>}
            </For>
          </select>
          <select
            value={subdivisionId()}
            disabled={subdivisions().length === 0}
            onChange={(event) => setSubdivisionId(event.currentTarget.value)}
          >
            <option value="">No subdivision</option>
            <For each={subdivisions()}>
              {(subdivision) => (
                <option value={subdivision.subdivision_id}>
                  {subdivision.country_flag} {subdivision.subdivision_name}
                </option>
              )}
            </For>
          </select>
          <NoticeView notice={notice()} />
          <button class="primary-button" disabled={running() || passwordsMismatch()} type="submit">
            {running() ? "Creating account" : "Create account"}
          </button>
          <button class="secondary-button" type="button" onClick={props.onSignIn}>
            Back to sign in
          </button>
        </form>
      </div>
    </section>
  );
}

interface SignInPageProps {
  readonly onForgotPassword: () => void;
  readonly onJoin: () => void;
  readonly onLogin: (response: LoginResponse) => void;
}

function SignInPage(props: SignInPageProps) {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [notice, setNotice] = createSignal<Notice>(emptyNotice);
  const [running, setRunning] = createSignal(false);

  const submit = async (event: SubmitEvent) => {
    event.preventDefault();
    setRunning(true);
    const result = await login({
      user_email: email().trim(),
      user_password: password()
    });
    setRunning(false);

    if (!result.ok || result.data === null) {
      setNotice({
        kind: "error",
        text: result.ok ? "Login response was empty." : result.error.message
      });
      return;
    }

    props.onLogin(result.data);
  };

  return (
    <section class="page-view auth-page">
      <div class="auth-card auth-card--narrow">
        <p class="eyebrow">Welcome back</p>
        <h1>Sign in</h1>
        <form class="flow-form" onSubmit={submit}>
          <input
            autocomplete="email"
            placeholder="Email"
            required
            type="email"
            value={email()}
            onInput={(event) => setEmail(event.currentTarget.value)}
          />
          <input
            autocomplete="current-password"
            placeholder="Password"
            required
            type="password"
            value={password()}
            onInput={(event) => setPassword(event.currentTarget.value)}
          />
          <button class="link-button" type="button" onClick={props.onForgotPassword}>
            Forgot password?
          </button>
          <NoticeView notice={notice()} />
          <button class="primary-button" disabled={running()} type="submit">
            {running() ? "Signing in" : "Sign in"}
          </button>
          <button class="secondary-button" type="button" onClick={props.onJoin}>
            Create account
          </button>
        </form>
      </div>
    </section>
  );
}

interface AccountPageProps {
  readonly countries: readonly ReferenceCountryResponse[];
  readonly languages: readonly ReferenceLanguageResponse[];
  readonly profile: MeResponse | null;
  readonly onSignIn: () => void;
  readonly onSignOut: () => void;
}

function AccountPage(props: AccountPageProps) {
  return (
    <section class="page-view account-layout">
      <Show
        when={props.profile}
        fallback={
          <div class="auth-card auth-card--narrow">
            <p class="eyebrow">Account</p>
            <h1>Sign in required</h1>
            <p class="hero-text">Your profile is available after you sign in.</p>
            <button class="primary-button" type="button" onClick={props.onSignIn}>
              Sign in
            </button>
          </div>
        }
      >
        {(profile) => (
          <>
            <div class="section-heading">
              <p class="eyebrow">Account</p>
              <h1>Your profile</h1>
            </div>
            <div class="profile-card">
              <div class="profile-hero">
                <div class="profile-avatar">{profile().user_info.user_name.slice(0, 1).toUpperCase()}</div>
                <div>
                  <h2>{profile().user_info.user_name}</h2>
                  <p>{profile().user_info.user_email}</p>
                </div>
              </div>
              <ProfileSummary countries={props.countries} languages={props.languages} profile={profile()} />
              <button class="secondary-button" type="button" onClick={props.onSignOut}>
                Sign out
              </button>
            </div>
          </>
        )}
      </Show>
    </section>
  );
}

interface RecoveryPageProps {
  readonly linkTokens: LinkTokens;
  readonly onSignIn: () => void;
}

function RecoveryPage(props: RecoveryPageProps) {
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");
  const [notice, setNotice] = createSignal<Notice>(emptyNotice);
  const [running, setRunning] = createSignal(false);
  const [emailVerified, setEmailVerified] = createSignal(false);

  const passwordsMismatch = createMemo(
    () => password().length > 0 && confirmPassword().length > 0 && password() !== confirmPassword()
  );

  createEffect(() => {
    const token = props.linkTokens.verificationToken;
    if (token === null || emailVerified()) {
      return;
    }

    setEmailVerified(true);
    void verifyEmail(token).then((result) => {
      setNotice(
        result.ok
          ? { kind: "success", text: "Email verified. You can now sign in." }
          : { kind: "error", text: result.error.message }
      );
    });
  });

  const requestReset = async (event: SubmitEvent) => {
    event.preventDefault();
    setRunning(true);
    const result = await requestPasswordReset({ user_email: email().trim() });
    setRunning(false);
    setNotice(
      result.ok
        ? { kind: "success", text: "Password reset email sent." }
        : { kind: "error", text: result.error.message }
    );
  };

  const applyReset = async (event: SubmitEvent) => {
    event.preventDefault();
    const token = props.linkTokens.resetToken;
    if (token === null) {
      setNotice({ kind: "error", text: "Open the reset link from your email to change password." });
      return;
    }

    if (passwordsMismatch()) {
      setNotice({ kind: "error", text: "Passwords do not match." });
      return;
    }

    setRunning(true);
    const result = await resetPassword({
      new_password: password(),
      password_reset_token: token
    });
    setRunning(false);
    setNotice(
      result.ok
        ? { kind: "success", text: "Password changed. You can now sign in." }
        : { kind: "error", text: result.error.message }
    );
  };

  return (
    <section class="page-view auth-page recovery-page">
      <div class="auth-card auth-card--narrow">
        <p class="eyebrow">Recovery</p>
        <h1>Reset password</h1>
        <Show
          when={props.linkTokens.resetToken !== null}
          fallback={
            <form class="flow-form" onSubmit={requestReset}>
              <p class="form-copy">Enter your email and we will send a reset link.</p>
              <input
                autocomplete="email"
                placeholder="Email"
                required
                type="email"
                value={email()}
                onInput={(event) => setEmail(event.currentTarget.value)}
              />
              <NoticeView notice={notice()} />
              <button class="primary-button" disabled={running()} type="submit">
                {running() ? "Sending" : "Send reset link"}
              </button>
              <button class="secondary-button" type="button" onClick={props.onSignIn}>
                Back to sign in
              </button>
            </form>
          }
        >
          <form class="flow-form" onSubmit={applyReset}>
            <p class="form-copy">Choose a new password for your account.</p>
            <input
              autocomplete="new-password"
              placeholder="New password"
              required
              type="password"
              value={password()}
              onInput={(event) => setPassword(event.currentTarget.value)}
            />
            <input
              autocomplete="new-password"
              aria-invalid={passwordsMismatch() ? "true" : "false"}
              placeholder="Re-enter new password"
              required
              type="password"
              value={confirmPassword()}
              onInput={(event) => setConfirmPassword(event.currentTarget.value)}
            />
            <Show when={passwordsMismatch()}>
              <p class="field-note field-note--error">Passwords do not match.</p>
            </Show>
            <NoticeView notice={notice()} />
            <button class="primary-button" disabled={running() || passwordsMismatch()} type="submit">
              {running() ? "Changing password" : "Change password"}
            </button>
            <button class="secondary-button" type="button" onClick={props.onSignIn}>
              Back to sign in
            </button>
          </form>
        </Show>
      </div>
    </section>
  );
}

interface ProfileSummaryProps {
  readonly countries: readonly ReferenceCountryResponse[];
  readonly languages: readonly ReferenceLanguageResponse[];
  readonly profile: MeResponse;
}

function ProfileSummary(props: ProfileSummaryProps) {
  const country = createMemo(() =>
    countryLabel(props.countries, props.profile.user_info.user_country)
  );
  const language = createMemo(() =>
    languageLabel(props.languages, props.profile.user_info.user_language)
  );

  return (
    <dl class="profile-grid">
      <div>
        <dt>Email</dt>
        <dd>{props.profile.user_info.user_email}</dd>
      </div>
      <div>
        <dt>Role</dt>
        <dd>{props.profile.claims.role_name}</dd>
      </div>
      <div>
        <dt>Country</dt>
        <dd>{country()}</dd>
      </div>
      <div>
        <dt>Language</dt>
        <dd>{language()}</dd>
      </div>
      <div>
        <dt>Email verified</dt>
        <dd>{props.profile.user_info.user_is_email_verified ? "Yes" : "No"}</dd>
      </div>
    </dl>
  );
}

function NoticeView(props: { readonly notice: Notice }) {
  return (
    <Show when={props.notice.kind !== "idle" && props.notice.text.length > 0}>
      <p class={`notice notice--${props.notice.kind}`}>{props.notice.text}</p>
    </Show>
  );
}

function initialTheme(): ThemeMode {
  const storedTheme = window.localStorage.getItem("preferred-theme");
  if (storedTheme === "light" || storedTheme === "dark") {
    return storedTheme;
  }

  if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
    return "dark";
  }

  return "light";
}

function readLinkTokens(): LinkTokens {
  const searchParams = new URLSearchParams(window.location.search);
  return {
    resetToken: searchParams.get("password_reset_token") ?? searchParams.get("token"),
    verificationToken: searchParams.get("email_validation_token_id")
  };
}

function resultData<TData>(result: ApiCallResult<TData> | undefined): TData | null {
  if (result === undefined || !result.ok) {
    return null;
  }

  return result.data;
}

function preferredCountry(
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

function findCountry(
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

function languagesWithPrimaryFirst(
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

function parseInteger(value: string): number | null {
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

function parseOptionalInteger(value: string): number | null | "invalid" {
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

function profileFromSession(session: LoginResponse | null): MeResponse | null {
  if (session === null) {
    return null;
  }

  return {
    claims: session.claims,
    user_info: {
      user_country: session.claims.user_country,
      user_created_at: session.claims.issued_at_iso,
      user_email: session.claims.user_email,
      user_id: session.claims.user_id,
      user_is_email_verified: session.claims.user_is_email_verified,
      user_language: session.claims.user_language,
      user_last_login_at: null,
      user_name: session.claims.user_name,
      user_subdivision: session.claims.user_subdivision,
      user_updated_at: session.claims.issued_at_iso
    }
  };
}

function countryLabel(countries: readonly ReferenceCountryResponse[], countryCode: number): string {
  const country = countries.find((candidate) => candidate.country_code === countryCode);
  if (country === undefined) {
    return countryCode.toString();
  }

  return `${country.country_flag} ${country.country_name}`;
}

function languageLabel(languages: readonly ReferenceLanguageResponse[], languageCode: number): string {
  const language = languages.find((candidate) => candidate.language_code === languageCode);
  if (language === undefined) {
    return languageCode.toString();
  }

  return language.language_name;
}
