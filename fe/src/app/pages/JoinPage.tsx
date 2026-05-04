import { createEffect, createMemo, createSignal, For, Show } from "solid-js";

import { checkIfUserExists, getCountrySubdivisions, signup } from "../../api/appApi";
import type {
  ReferenceCountryResponse,
  ReferenceLanguageResponse,
  ReferenceSubdivisionResponse,
  SignupRequest,
  SignupResponse,
  SignupRoleType
} from "../../api/types";
import {
  findCountry,
  languagesWithPrimaryFirst,
  parseInteger,
  parseOptionalInteger,
  preferredCountry
} from "../helpers";
import { NoticeView } from "../shared/Feedback";
import { emptyNotice, type Notice } from "../shared/types";
import { SignupVerificationPending } from "./SignupVerificationPending";

interface JoinPageProps {
  readonly countries: readonly ReferenceCountryResponse[];
  readonly languages: readonly ReferenceLanguageResponse[];
  readonly onSignIn: () => void;
}

export function JoinPage(props: JoinPageProps) {
  const [userName, setUserName] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");
  const [signupRole, setSignupRole] = createSignal<SignupRoleType>("user");
  const [countryCode, setCountryCode] = createSignal("");
  const [languageCode, setLanguageCode] = createSignal("");
  const [subdivisionId, setSubdivisionId] = createSignal("");
  const [subdivisions, setSubdivisions] = createSignal<readonly ReferenceSubdivisionResponse[]>([]);
  const [createdAccount, setCreatedAccount] = createSignal<SignupResponse | null>(null);
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
      user_role: signupRole(),
      user_subdivision: userSubdivision
    };
    const result = await signup(body);
    setRunning(false);

    if (!result.ok) {
      setNotice({ kind: "error", text: result.error.message });
      return;
    }
    if (result.data === null) {
      setNotice({ kind: "error", text: "Signup response was empty." });
      return;
    }

    setCreatedAccount(result.data);
    setNotice(emptyNotice);
  };

  return (
    <section class="page-view auth-page">
      <div class="auth-card">
        <Show
          when={createdAccount()}
          fallback={
            <>
              <p class="eyebrow">New account</p>
              <h1>Create your account</h1>
              <form class="flow-form" onSubmit={submit}>
                <input
                  aria-label="Username"
                  autocomplete="username"
                  placeholder="Username"
                  required
                  value={userName()}
                  onInput={(event) => setUserName(event.currentTarget.value)}
                />
                <input
                  aria-label="Email"
                  autocomplete="email"
                  placeholder="Email"
                  required
                  type="email"
                  value={email()}
                  onInput={(event) => setEmail(event.currentTarget.value)}
                />
                <input
                  aria-label="Password"
                  autocomplete="new-password"
                  placeholder="Password"
                  required
                  type="password"
                  value={password()}
                  onInput={(event) => setPassword(event.currentTarget.value)}
                />
                <input
                  aria-invalid={passwordsMismatch() ? "true" : "false"}
                  aria-label="Re-enter password"
                  autocomplete="new-password"
                  placeholder="Re-enter password"
                  required
                  type="password"
                  value={confirmPassword()}
                  onInput={(event) => setConfirmPassword(event.currentTarget.value)}
                />
                <Show when={passwordsMismatch()}>
                  <p class="field-note field-note--error">Passwords do not match.</p>
                </Show>
                <fieldset class="segmented-control">
                  <legend>Account type</legend>
                  <div class="segmented-control__options">
                    <SignupRoleOption
                      active={signupRole() === "user"}
                      label="User"
                      value="user"
                      onSelect={() => setSignupRole("user")}
                    />
                    <SignupRoleOption
                      active={signupRole() === "service_provider"}
                      label="Service provider"
                      value="service_provider"
                      onSelect={() => setSignupRole("service_provider")}
                    />
                  </div>
                </fieldset>
                <select
                  aria-label="Country"
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
                  aria-label="Language"
                  required
                  value={languageCode()}
                  onChange={(event) => setLanguageCode(event.currentTarget.value)}
                >
                  <option value="">Select language</option>
                  <For each={orderedLanguages()}>
                    {(language) => (
                      <option value={language.language_code}>{language.language_name}</option>
                    )}
                  </For>
                </select>
                <select
                  aria-label="Subdivision"
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
                <button
                  class="primary-button"
                  disabled={running() || passwordsMismatch()}
                  type="submit"
                >
                  {running() ? "Creating account" : "Create account"}
                </button>
                <button class="secondary-button" type="button" onClick={props.onSignIn}>
                  Back to sign in
                </button>
              </form>
            </>
          }
        >
          {(account) => <SignupVerificationPending account={account()} onSignIn={props.onSignIn} />}
        </Show>
      </div>
    </section>
  );
}

function SignupRoleOption(props: {
  readonly active: boolean;
  readonly label: string;
  readonly value: SignupRoleType;
  readonly onSelect: () => void;
}) {
  return (
    <label class={props.active ? "segmented-option segmented-option--active" : "segmented-option"}>
      <input
        checked={props.active}
        name="signup-role"
        type="radio"
        value={props.value}
        onChange={props.onSelect}
      />
      <span>{props.label}</span>
    </label>
  );
}
