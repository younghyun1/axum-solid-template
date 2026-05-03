import { Show } from "solid-js";

import type { Notice } from "./types";

export function NoticeView(props: { readonly notice: Notice }) {
  return (
    <Show when={props.notice.kind !== "idle" && props.notice.text.length > 0}>
      <p class={`notice notice--${props.notice.kind}`}>{props.notice.text}</p>
    </Show>
  );
}

export function SpinnerStatus(props: { readonly text: string }) {
  return (
    <div class="spinner-status" role="status">
      <span class="spinner" aria-hidden="true" />
      <span>{props.text}</span>
    </div>
  );
}
