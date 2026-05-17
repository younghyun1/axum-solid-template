interface HomePageProps {
  readonly isSignedIn: boolean;
  readonly serviceOnline: boolean;
  readonly onBrowseProviders: () => void;
  readonly onCreateAccount: () => void;
  readonly onSignIn: () => void;
}

export function HomePage(props: HomePageProps) {
  return (
    <section class="template-home" aria-label="Provider marketplace intro">
      <div class="template-home__media" aria-hidden="true">
        <img
          src="https://images.unsplash.com/photo-1497366754035-f200968a6e72?auto=format&fit=crop&w=1800&q=80"
          alt=""
        />
        <img
          src="https://images.unsplash.com/photo-1497366811353-6870744d04b2?auto=format&fit=crop&w=1800&q=80"
          alt=""
        />
        <img
          src="https://images.unsplash.com/photo-1517245386807-bb43f82c33c4?auto=format&fit=crop&w=1800&q=80"
          alt=""
        />
        <img
          src="https://images.unsplash.com/photo-1556761175-b413da4baf72?auto=format&fit=crop&w=1800&q=80"
          alt=""
        />
      </div>

      <div class="template-home__content">
        <div class="template-home__status" aria-label="Service status">
          <span
            aria-hidden="true"
            class={props.serviceOnline ? "status-light status-light--ok" : "status-light"}
          />
          <span>{props.serviceOnline ? "Directory online" : "Checking directory"}</span>
        </div>

        <h1>Find trusted service providers.</h1>
        <p>
          Browse published profiles, compare service areas, read provider updates, and manage
          account access from one SolidJS marketplace surface.
        </p>

        <div class="template-home__actions">
          <button class="template-ghost-link" type="button" onClick={props.onBrowseProviders}>
            Directory
          </button>
          <button class="template-ghost-link" type="button" onClick={props.onBrowseProviders}>
            Live sessions
          </button>
          <button class="template-ghost-link" type="button" onClick={props.onBrowseProviders}>
            Guides
          </button>
          <button class="template-ghost-link" type="button" onClick={props.onBrowseProviders}>
            Events
          </button>
        </div>

        <div class="template-home__account">
          <button class="template-outline-button" type="button" onClick={props.onSignIn}>
            {props.isSignedIn ? "Account" : "Log in"}
          </button>
          <button class="template-solid-button" type="button" onClick={props.onCreateAccount}>
            Sign up
          </button>
        </div>
      </div>
    </section>
  );
}
