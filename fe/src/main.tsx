import { render } from "solid-js/web";
import { Route, Router } from "@solidjs/router";

import { App } from "./app/App";
import "./styles/base.css";

const root = document.getElementById("root");

if (root instanceof HTMLElement) {
  render(
    () => (
      <Router>
        <Route path="*404" component={App} />
      </Router>
    ),
    root
  );
} else {
  console.error("Solid root element was not found");
}
