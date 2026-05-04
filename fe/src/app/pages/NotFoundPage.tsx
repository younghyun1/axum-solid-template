interface NotFoundPageProps {
  readonly onHome: () => void;
}

export function NotFoundPage(props: NotFoundPageProps) {
  return (
    <section class="page-view auth-page">
      <div class="auth-card auth-card--narrow">
        <p class="eyebrow">Not found</p>
        <h1>Page not found</h1>
        <p class="form-copy">The requested page is not available.</p>
        <button class="primary-button" type="button" onClick={props.onHome}>
          Home
        </button>
      </div>
    </section>
  );
}
