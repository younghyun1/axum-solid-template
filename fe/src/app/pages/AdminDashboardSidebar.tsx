export type AdminDashboardSection = "verification" | "database";

interface AdminDashboardSidebarProps {
  readonly activeSection: AdminDashboardSection;
  readonly onSectionChange: (section: AdminDashboardSection) => void;
}

export function AdminDashboardSidebar(props: AdminDashboardSidebarProps) {
  return (
    <aside class="admin-sidebar" aria-label="Admin sections">
      <button
        classList={{ "admin-nav-button--active": props.activeSection === "verification" }}
        class="admin-nav-button"
        type="button"
        onClick={() => props.onSectionChange("verification")}
      >
        Verification
      </button>
      <button
        classList={{ "admin-nav-button--active": props.activeSection === "database" }}
        class="admin-nav-button"
        type="button"
        onClick={() => props.onSectionChange("database")}
      >
        Database
      </button>
    </aside>
  );
}
