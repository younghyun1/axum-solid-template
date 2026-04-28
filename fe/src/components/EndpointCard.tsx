import { createSignal, For, Show, type Accessor, type Setter } from "solid-js";

import { clientFailure, requestApi } from "../api/client";
import type { ApiDemo } from "../api/backendApi";
import type { FormValues } from "../api/forms";
import type { ApiCallResult, JsonObject, LoginResponse } from "../api/types";
import { JsonBlock } from "./JsonBlock";

interface EndpointCardProps {
  readonly demo: ApiDemo;
  readonly token: Accessor<string>;
  readonly onToken: Setter<string>;
}

export function EndpointCard(props: EndpointCardProps) {
  const [values, setValues] = createSignal<FormValues>(initialValues(props.demo));
  const [result, setResult] = createSignal<ApiCallResult<unknown> | null>(null);
  const [running, setRunning] = createSignal(false);

  const submit = async (event: SubmitEvent) => {
    event.preventDefault();
    const buildResult = props.demo.buildRequest(values());
    if (!buildResult.ok) {
      setResult(clientFailure(buildResult.message));
      return;
    }

    const token = props.token().trim();
    if (props.demo.endpoint.auth === "required" && token.length === 0) {
      setResult(clientFailure("Bearer token is required"));
      return;
    }

    setRunning(true);
    const apiRequestOptions = {
      method: buildResult.request.method,
      path: buildResult.request.path,
      ...(buildResult.request.body !== undefined ? { body: buildResult.request.body } : {}),
      ...(buildResult.request.query !== undefined ? { query: buildResult.request.query } : {}),
      ...(props.demo.endpoint.auth !== "none" && token.length > 0 ? { token } : {})
    };
    const apiResult = await requestApi<unknown, JsonObject | undefined>(apiRequestOptions);
    setRunning(false);
    setResult(apiResult);

    if (apiResult.ok && props.demo.endpoint.id === "login" && isLoginResponse(apiResult.data)) {
      props.onToken(apiResult.data.access_token);
    }
  };

  return (
    <article class="endpoint-card">
      <div class="endpoint-card__heading">
        <div>
          <h2>{props.demo.endpoint.title}</h2>
          <p>
            <span class="method-pill">{props.demo.endpoint.method}</span>
            <code>{props.demo.endpoint.path}</code>
          </p>
        </div>
        <span class="auth-pill">{props.demo.endpoint.auth}</span>
      </div>

      <form class="endpoint-form" onSubmit={submit}>
        <For each={props.demo.fields}>
          {(field) => (
            <label class="field-control">
              <span>
                {field.label}
                <Show when={field.optional}>
                  <small> optional</small>
                </Show>
              </span>
              <input
                autocomplete="off"
                inputmode={field.kind === "number" ? "numeric" : "text"}
                onInput={(event) =>
                  setValues((current) => ({
                    ...current,
                    [field.key]: event.currentTarget.value
                  }))
                }
                spellcheck={false}
                type={field.kind}
                value={values()[field.key] ?? ""}
              />
            </label>
          )}
        </For>

        <button class="primary-button" disabled={running()} type="submit">
          {running() ? "Running" : "Run"}
        </button>
      </form>

      <Show when={result()}>
        {(resolvedResult) => (
          <div class={resolvedResult().ok ? "result result--ok" : "result result--error"}>
            <p class="result__status">{resultLabel(resolvedResult())}</p>
            <JsonBlock value={resultPayload(resolvedResult())} />
          </div>
        )}
      </Show>
    </article>
  );
}

function initialValues(demo: ApiDemo): FormValues {
  const values: FormValues = {};
  for (const field of demo.fields) {
    values[field.key] = field.initialValue;
  }
  return values;
}

function resultLabel(result: ApiCallResult<unknown>): string {
  if (result.ok) {
    return `HTTP ${result.status}`;
  }

  if (result.status === null) {
    return result.error.message;
  }

  return `HTTP ${result.status}: ${result.error.message}`;
}

function resultPayload(result: ApiCallResult<unknown>): unknown {
  if (result.ok) {
    return result.envelope;
  }

  return result.envelope ?? {
    error: result.error
  };
}

function isLoginResponse(value: unknown): value is LoginResponse {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }

  return "access_token" in value && typeof value.access_token === "string";
}
