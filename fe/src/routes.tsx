import { Route, Router } from "@solidjs/router";
import { type Component } from "solid-js";
import { AppShell } from "./components/AppShell";
import { HealthPage } from "./pages/HealthPage";
import { HomePage } from "./pages/HomePage";
import { NotFoundPage } from "./pages/NotFoundPage";

export const AppRouter: Component = () => {
  return (
    <Router root={AppShell}>
      <Route path="/" component={HomePage} />
      <Route path="/health" component={HealthPage} />
      <Route path="*404" component={NotFoundPage} />
    </Router>
  );
};

