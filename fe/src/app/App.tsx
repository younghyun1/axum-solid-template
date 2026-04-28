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
  HealthcheckResponse,
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
  const [activePage, setActivePage] = createSignal<PageId>("home");
  const [theme, setTheme] = createSignal<ThemeMode>(initialTheme());
  const [displayLanguage, setDisplayLanguage] = createSignal("en");
  const [token, setToken] = createSignal("");
  const [session, setSession] = createSignal<LoginResponse | null>(null);
  const [profile, setProfile] = createSignal<MeResponse | null>(null);

  const [countriesResult] = createResource(getCountries);
  const [languagesResult] = createResource(getLanguages);
  const [healthResult, { refetch: refetchHealth }] = createResource(getHealthcheck);

  const countries = createMemo(() => resultData(countriesResult()) ?? []);
  const languages = createMemo(() => resultData(languagesResult()) ?? []);
  const health = createMemo(() => resultData(healthResult()));

  createEffect(() => {
    const selectedTheme = theme();
    document.documentElement.dataset["theme"] = selectedTheme;
    window.localStorage.setItem("preferred-theme", selectedTheme);
  });

  const authLabel = createMemo(() => {
    const currentSession = session();
    if (currentSession !== null) {
      return currentSession.claims.user_name;
    }

    if (token().trim().length > 0) {
      return "Token active";
    }

    return "Guest";
  });

  const toggleTheme = () => {
    setTheme((current) => (current === "light" ? "dark" : "light"));
  };

  const clearSession = () => {
    setSession(null);
    setProfile(null);
    setToken("");
  };

  return (
    <div class="app-shell">
      <header class="top-bar">
        <button class="brand-button" type="button" onClick={() => setActivePage("home")}>
          Orville
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
          <button
            aria-label="Toggle color theme"
            class="utility-button utility-button--icon"
            type="button"
            onClick={toggleTheme}
          >
            {theme() === "light" ? "🌙" : "☀️"}
          </button>
          <details class="auth-menu">
            <summary>
              <span>{authLabel()}</span>
              <small>{token().trim().length > 0 ? "Signed session" : "No session"}</small>
            </summary>
            <label class="token-control">
              <span>Access token</span>
              <input
                autocomplete="off"
                inputmode="text"
                onInput={(event) => {
                  setToken(event.currentTarget.value);
                  setSession(null);
                  setProfile(null);
                }}
                placeholder="Paste token"
                spellcheck={false}
                type="text"
                value={token()}
              />
            </label>
          </details>
        </div>
      </header>

      <main>
        <Show when={activePage() === "home"}>
          <HomePage
            health={health()}
            healthLoading={healthResult.loading}
            onRefreshHealth={() => refetchHealth()}
            onStart={() => setActivePage("join")}
          />
        </Show>

        <Show when={activePage() === "join"}>
          <JoinPage countries={countries()} languages={languages()} onSignedUp={() => setActivePage("signin")} />
        </Show>

        <Show when={activePage() === "signin"}>
          <SignInPage
            onLogin={(response) => {
              setToken(response.access_token);
              setSession(response);
              setProfile(null);
              setActivePage("account");
            }}
          />
        </Show>

        <Show when={activePage() === "account"}>
          <AccountPage
            countries={countries()}
            languages={languages()}
            profile={profile()}
            session={session()}
            token={token()}
            onClear={clearSession}
            onProfile={setProfile}
          />
        </Show>

        <Show when={activePage() === "recovery"}>
          <RecoveryPage />
        </Show>
      </main>
    </div>
  );
}

interface HomePageProps {
  readonly health: HealthcheckResponse | null;
  readonly healthLoading: boolean;
  readonly onRefreshHealth: () => void;
  readonly onStart: () => void;
}

function HomePage(props: HomePageProps) {
  const readiness = createMemo(() => {
    if (props.healthLoading) {
      return "Checking";
    }

    if (props.health?.accepting_traffic === true) {
      return "Online";
    }

    return "Unavailable";
  });

  return (
    <section class="page-view home-hero">
      <div class="hero-copy">
        <p class="eyebrow">Secure customer access</p>
        <h1>Account access that feels fast, clear, and ready.</h1>
        <p class="hero-text">
          Create an account, sign in, recover credentials, and check your session without touching
          raw API payloads.
        </p>
        <div class="hero-actions">
          <button class="primary-button" type="button" onClick={props.onStart}>
            Create account
          </button>
          <button class="secondary-button" type="button" onClick={props.onRefreshHealth}>
            Refresh status
          </button>
        </div>
      </div>
      <div class="status-panel">
        <p class="eyebrow">Service</p>
        <h2>{readiness()}</h2>
        <dl class="summary-list">
          <div>
            <dt>Traffic</dt>
            <dd>{props.health?.accepting_traffic === true ? "Accepting" : "Pending"}</dd>
          </div>
          <div>
            <dt>Envelope</dt>
            <dd>Synced</dd>
          </div>
        </dl>
      </div>
    </section>
  );
}

interface JoinPageProps {
  readonly countries: readonly ReferenceCountryResponse[];
  readonly languages: readonly ReferenceLanguageResponse[];
  readonly onSignedUp: () => void;
}

function JoinPage(props: JoinPageProps) {
  const [userName, setUserName] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
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
    const country = selectedCountry();
    const userCountry = parseInteger(countryCode());
    const userLanguage = parseInteger(languageCode());
    const userSubdivision = parseOptionalInteger(subdivisionId());

    if (country === null || userCountry === null || userLanguage === null) {
      setNotice({ kind: "error", text: "Choose a country and language." });
      return;
    }

    if (userSubdivision === "invalid") {
      setNotice({ kind: "error", text: "Choose a valid subdivision." });
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

    setNotice({ kind: "success", text: `Verification email queued for ${result.data?.user_email ?? email()}.` });
    props.onSignedUp();
  };

  return (
    <section class="page-view form-layout">
      <div class="section-heading">
        <p class="eyebrow">New account</p>
        <h1>Create your account</h1>
      </div>
      <form class="flow-panel" onSubmit={submit}>
        <div class="field-grid">
          <label class="field-control">
            <span>Username</span>
            <input autocomplete="username" required value={userName()} onInput={(event) => setUserName(event.currentTarget.value)} />
          </label>
          <label class="field-control">
            <span>Email</span>
            <input autocomplete="email" required type="email" value={email()} onInput={(event) => setEmail(event.currentTarget.value)} />
          </label>
          <label class="field-control">
            <span>Password</span>
            <input autocomplete="new-password" required type="password" value={password()} onInput={(event) => setPassword(event.currentTarget.value)} />
          </label>
          <label class="field-control">
            <span>Country</span>
            <select required value={countryCode()} onChange={(event) => setCountryCode(event.currentTarget.value)}>
              <option value="">Select country</option>
              <For each={props.countries}>
                {(country) => (
                  <option value={country.country_code}>
                    {country.country_flag} {country.country_name}
                  </option>
                )}
              </For>
            </select>
          </label>
          <label class="field-control">
            <span>Subdivision</span>
            <select value={subdivisionId()} onChange={(event) => setSubdivisionId(event.currentTarget.value)}>
              <option value="">None</option>
              <For each={subdivisions()}>
                {(subdivision) => (
                  <option value={subdivision.subdivision_id}>
                    {subdivision.country_flag} {subdivision.subdivision_name}
                  </option>
                )}
              </For>
            </select>
          </label>
          <label class="field-control">
            <span>Primary language</span>
            <select required value={languageCode()} onChange={(event) => setLanguageCode(event.currentTarget.value)}>
              <For each={orderedLanguages()}>
                {(language) => (
                  <option value={language.language_code}>
                    {language.language_name}
                  </option>
                )}
              </For>
            </select>
          </label>
        </div>
        <button class="primary-button" disabled={running()} type="submit">
          {running() ? "Creating" : "Create account"}
        </button>
        <NoticeView notice={notice()} />
      </form>
    </section>
  );
}

interface SignInPageProps {
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
      setNotice({ kind: "error", text: result.ok ? "Login response was empty." : result.error.message });
      return;
    }

    setNotice({ kind: "success", text: `Signed in as ${result.data.claims.user_name}.` });
    props.onLogin(result.data);
  };

  return (
    <section class="page-view form-layout form-layout--narrow">
      <div class="section-heading">
        <p class="eyebrow">Welcome back</p>
        <h1>Sign in</h1>
      </div>
      <form class="flow-panel" onSubmit={submit}>
        <label class="field-control">
          <span>Email</span>
          <input autocomplete="email" required type="email" value={email()} onInput={(event) => setEmail(event.currentTarget.value)} />
        </label>
        <label class="field-control">
          <span>Password</span>
          <input autocomplete="current-password" required type="password" value={password()} onInput={(event) => setPassword(event.currentTarget.value)} />
        </label>
        <button class="primary-button" disabled={running()} type="submit">
          {running() ? "Signing in" : "Sign in"}
        </button>
        <NoticeView notice={notice()} />
      </form>
    </section>
  );
}

interface AccountPageProps {
  readonly countries: readonly ReferenceCountryResponse[];
  readonly languages: readonly ReferenceLanguageResponse[];
  readonly profile: MeResponse | null;
  readonly session: LoginResponse | null;
  readonly token: string;
  readonly onClear: () => void;
  readonly onProfile: (profile: MeResponse | null) => void;
}

function AccountPage(props: AccountPageProps) {
  const [notice, setNotice] = createSignal<Notice>(emptyNotice);
  const [running, setRunning] = createSignal(false);

  const loadProfile = async () => {
    if (props.token.trim().length === 0) {
      setNotice({ kind: "error", text: "Sign in or paste an access token first." });
      return;
    }

    setRunning(true);
    const result = await me(props.token);
    setRunning(false);

    if (!result.ok || result.data === null) {
      setNotice({ kind: "error", text: result.ok ? "Profile response was empty." : result.error.message });
      return;
    }

    props.onProfile(result.data);
    setNotice({ kind: "success", text: "Profile refreshed." });
  };

  const signOut = async () => {
    if (props.token.trim().length > 0) {
      await logout(props.token);
    }
    props.onClear();
    setNotice({ kind: "success", text: "Signed out." });
  };

  const visibleProfile = createMemo(() => props.profile ?? profileFromSession(props.session));

  return (
    <section class="page-view account-layout">
      <div class="section-heading">
        <p class="eyebrow">Session</p>
        <h1>Your account</h1>
      </div>
      <div class="flow-panel">
        <div class="action-row">
          <button class="primary-button" disabled={running()} type="button" onClick={loadProfile}>
            {running() ? "Refreshing" : "Refresh profile"}
          </button>
          <button class="secondary-button" type="button" onClick={signOut}>
            Sign out
          </button>
        </div>
        <Show when={visibleProfile()} fallback={<p class="empty-state">No active profile.</p>}>
          {(resolvedProfile) => (
            <ProfileSummary
              countries={props.countries}
              languages={props.languages}
              profile={resolvedProfile()}
            />
          )}
        </Show>
        <NoticeView notice={notice()} />
      </div>
    </section>
  );
}

function RecoveryPage() {
  const [email, setEmail] = createSignal("");
  const [resetToken, setResetToken] = createSignal("");
  const [newPassword, setNewPassword] = createSignal("");
  const [verificationToken, setVerificationToken] = createSignal("");
  const [notice, setNotice] = createSignal<Notice>(emptyNotice);
  const [running, setRunning] = createSignal(false);

  const requestReset = async (event: SubmitEvent) => {
    event.preventDefault();
    setRunning(true);
    const result = await requestPasswordReset({ user_email: email().trim() });
    setRunning(false);
    setNotice(result.ok ? { kind: "success", text: "Password reset email queued." } : { kind: "error", text: result.error.message });
  };

  const applyReset = async (event: SubmitEvent) => {
    event.preventDefault();
    setRunning(true);
    const result = await resetPassword({
      new_password: newPassword(),
      password_reset_token: resetToken().trim()
    });
    setRunning(false);
    setNotice(result.ok ? { kind: "success", text: "Password changed." } : { kind: "error", text: result.error.message });
  };

  const verify = async (event: SubmitEvent) => {
    event.preventDefault();
    setRunning(true);
    const result = await verifyEmail(verificationToken().trim());
    setRunning(false);
    setNotice(result.ok ? { kind: "success", text: "Email verified." } : { kind: "error", text: result.error.message });
  };

  return (
    <section class="page-view form-layout">
      <div class="section-heading">
        <p class="eyebrow">Recovery</p>
        <h1>Secure recovery</h1>
      </div>
      <div class="recovery-grid">
        <form class="flow-panel" onSubmit={requestReset}>
          <h2>Reset request</h2>
          <label class="field-control">
            <span>Email</span>
            <input required type="email" value={email()} onInput={(event) => setEmail(event.currentTarget.value)} />
          </label>
          <button class="primary-button" disabled={running()} type="submit">
            Send reset
          </button>
        </form>
        <form class="flow-panel" onSubmit={applyReset}>
          <h2>Change password</h2>
          <label class="field-control">
            <span>Reset token</span>
            <input required value={resetToken()} onInput={(event) => setResetToken(event.currentTarget.value)} />
          </label>
          <label class="field-control">
            <span>New password</span>
            <input required type="password" value={newPassword()} onInput={(event) => setNewPassword(event.currentTarget.value)} />
          </label>
          <button class="primary-button" disabled={running()} type="submit">
            Change password
          </button>
        </form>
        <form class="flow-panel" onSubmit={verify}>
          <h2>Email verification</h2>
          <label class="field-control">
            <span>Verification token</span>
            <input required value={verificationToken()} onInput={(event) => setVerificationToken(event.currentTarget.value)} />
          </label>
          <button class="primary-button" disabled={running()} type="submit">
            Verify email
          </button>
        </form>
      </div>
      <NoticeView notice={notice()} />
    </section>
  );
}

interface ProfileSummaryProps {
  readonly countries: readonly ReferenceCountryResponse[];
  readonly languages: readonly ReferenceLanguageResponse[];
  readonly profile: MeResponse;
}

function ProfileSummary(props: ProfileSummaryProps) {
  const country = createMemo(() => countryLabel(props.countries, props.profile.user_info.user_country));
  const language = createMemo(() =>
    languageLabel(props.languages, props.profile.user_info.user_language)
  );

  return (
    <dl class="profile-grid">
      <div>
        <dt>Name</dt>
        <dd>{props.profile.user_info.user_name}</dd>
      </div>
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
        <dt>Verified</dt>
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

  const primaryLanguage = languages.find((language) => language.language_code === primaryLanguageCode);
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
