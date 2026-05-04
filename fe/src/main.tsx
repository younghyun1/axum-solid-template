import { render } from "solid-js/web";
import { Router } from "@solidjs/router";

import { App } from "./app/App";
import "./styles/base.css";

const root = document.getElementById("root");

if (root instanceof HTMLElement) {
  render(
    () => (
      <Router>
        <App />
      </Router>
    ),
    root
  );
} else {
  console.error("Solid root element was not found");
}
