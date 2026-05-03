interface HomePageProps {
  readonly isSignedIn: boolean;
  readonly serviceOnline: boolean;
  readonly onCreateAccount: () => void;
  readonly onSignIn: () => void;
}

export function HomePage(props: HomePageProps) {
  return (
    <section class="page-view landing-layout">
      <div class="landing-copy">
        <p class="eyebrow">Rust-Solid-Template</p>
        <h1>Sign in, create an account, or recover access.</h1>
        <p class="hero-text">A focused account flow for users and service providers.</p>
        <div class="hero-actions">
          <button class="primary-button" type="button" onClick={props.onCreateAccount}>
            Create account
          </button>
          <button class="secondary-button" type="button" onClick={props.onSignIn}>
            Sign in
          </button>
        </div>
      </div>

      <aside class="portal-panel" aria-label="Service status">
        <div class="portal-panel__header">
          <span
            aria-hidden="true"
            class={props.serviceOnline ? "status-light status-light--ok" : "status-light"}
          />
          <div>
            <p class="eyebrow">Service status</p>
            <h2>{props.serviceOnline ? "All systems available" : "Checking status..."}</h2>
          </div>
        </div>
        <p class="form-copy">
          {props.isSignedIn
            ? "You are signed in. Manage your profile from the account view."
            : "Sign in or create an account to get started."}
        </p>
      </aside>
    </section>
  );
}
