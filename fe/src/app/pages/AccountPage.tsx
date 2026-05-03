import { createMemo, Show } from "solid-js";

import type { MeResponse, ReferenceCountryResponse, ReferenceLanguageResponse } from "../../api/types";
import { countryLabel, languageLabel } from "../helpers";

interface AccountPageProps {
  readonly countries: readonly ReferenceCountryResponse[];
  readonly languages: readonly ReferenceLanguageResponse[];
  readonly profile: MeResponse | null;
  readonly onSignIn: () => void;
  readonly onSignOut: () => void;
}

export function AccountPage(props: AccountPageProps) {
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
                <div class="profile-avatar">
                  {profile().user_info.user_name.slice(0, 1).toUpperCase()}
                </div>
                <div>
                  <h2>{profile().user_info.user_name}</h2>
                  <p>{profile().user_info.user_email}</p>
                </div>
              </div>
              <ProfileSummary
                countries={props.countries}
                languages={props.languages}
                profile={profile()}
              />
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

function ProfileSummary(props: {
  readonly countries: readonly ReferenceCountryResponse[];
  readonly languages: readonly ReferenceLanguageResponse[];
  readonly profile: MeResponse;
}) {
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
