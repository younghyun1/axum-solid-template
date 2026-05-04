interface AdminDatabasePanelProps {
  readonly running: boolean;
  readonly onHome: () => void;
  readonly onResetDatabase: () => void;
}

export function AdminDatabasePanel(props: AdminDatabasePanelProps) {
  return (
    <section class="auth-card admin-form admin-form--narrow">
      <h2>Database</h2>
      <div class="admin-toolbar">
        <button
          class="danger-button"
          disabled={props.running}
          type="button"
          onClick={props.onResetDatabase}
        >
          Reset database
        </button>
        <button class="secondary-button" type="button" onClick={props.onHome}>
          Home
        </button>
      </div>
    </section>
  );
}
