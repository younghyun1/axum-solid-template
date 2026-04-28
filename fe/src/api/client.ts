import type {
  ApiCallResult,
  ApiEnvelope,
  ApiErrorBody,
  ApiErrorLevel,
  ApiFailureKind,
  ApiMeta,
  ApiMethod,
  JsonObject,
  NormalizedApiError
} from "./types";

export interface ApiRequestOptions<TBody extends JsonObject | undefined> {
  readonly method: ApiMethod;
  readonly path: string;
  readonly baseUrl?: string;
  readonly body?: TBody;
  readonly query?: Readonly<Record<string, string>>;
  readonly token?: string;
  readonly fetcher?: typeof fetch;
  readonly signal?: AbortSignal;
}

export async function requestApi<TData, TBody extends JsonObject | undefined = undefined>(
  options: ApiRequestOptions<TBody>
): Promise<ApiCallResult<TData>> {
  const fetcher = options.fetcher ?? globalThis.fetch;
  if (typeof fetcher !== "function") {
    return clientFailure("Fetch API is unavailable in this environment");
  }

  const url = buildUrl(options.path, options.baseUrl, options.query);
  const headers = new Headers({
    Accept: "application/json"
  });
  const trimmedToken = options.token?.trim() ?? "";

  if (trimmedToken.length > 0) {
    headers.set("Authorization", `Bearer ${trimmedToken}`);
  }

  const requestInit: RequestInit = {
    headers,
    method: options.method
  };

  if (options.signal !== undefined) {
    requestInit.signal = options.signal;
  }

  if (options.body !== undefined) {
    headers.set("Content-Type", "application/json");
    requestInit.body = JSON.stringify(options.body);
  }

  let response: Response;
  try {
    response = await fetcher(url, requestInit);
  } catch (cause: unknown) {
    return failure("network", null, null, "Network request failed", cause);
  }

  const payload = await readJson(response);
  if (!isApiEnvelope(payload)) {
    return failure(
      "invalid_response",
      response.status,
      null,
      "Response did not match the API envelope"
    );
  }

  if (!response.ok || !payload.success) {
    return {
      ok: false,
      status: response.status,
      envelope: payload,
      error: normalizeApiError(response.status, payload)
    };
  }

  return {
    ok: true,
    status: response.status,
    envelope: payload as ApiEnvelope<TData>,
    data: payload.data as TData | null,
    meta: payload.meta
  };
}

export function clientFailure(message: string): ApiCallResult<never> {
  return failure("client", null, null, message);
}

export function buildUrl(
  path: string,
  baseUrl: string | undefined,
  query: Readonly<Record<string, string>> | undefined
): string {
  const base = baseUrl ?? defaultBaseUrl();
  const url = new URL(path, base);

  if (query !== undefined) {
    for (const [key, value] of Object.entries(query)) {
      url.searchParams.set(key, value);
    }
  }

  return url.toString();
}

export function isApiEnvelope(value: unknown): value is ApiEnvelope<unknown> {
  if (!isRecord(value)) {
    return false;
  }

  if (typeof value["success"] !== "boolean") {
    return false;
  }

  if (!("data" in value) || !("error" in value)) {
    return false;
  }

  if (!isApiMeta(value["meta"])) {
    return false;
  }

  const errorValue = value["error"];
  return errorValue === null || isApiErrorBody(errorValue);
}

export function normalizeApiError(
  status: number | null,
  envelope: ApiEnvelope<unknown> | null,
  fallbackMessage = "API request failed"
): NormalizedApiError {
  if (envelope?.error !== null && envelope?.error !== undefined) {
    return {
      kind: "backend",
      message: backendErrorMessage(envelope.error),
      status,
      backendError: envelope.error,
      meta: envelope.meta
    };
  }

  if (status !== null) {
    return {
      kind: "http",
      message: `${fallbackMessage} with HTTP ${status}`,
      status,
      backendError: null,
      meta: envelope?.meta ?? null
    };
  }

  return {
    kind: "network",
    message: fallbackMessage,
    status: null,
    backendError: null,
    meta: envelope?.meta ?? null
  };
}

function defaultBaseUrl(): string {
  if (typeof window === "undefined") {
    return "http://127.0.0.1";
  }

  return window.location.origin;
}

async function readJson(response: Response): Promise<unknown> {
  const text = await response.text();
  if (text.trim().length === 0) {
    return null;
  }

  try {
    return JSON.parse(text) as unknown;
  } catch {
    return null;
  }
}

function failure<TData>(
  kind: ApiFailureKind,
  status: number | null,
  envelope: ApiEnvelope<unknown> | null,
  message: string,
  cause?: unknown
): ApiCallResult<TData> {
  const baseError: NormalizedApiError = {
    kind,
    message,
    status,
    backendError: envelope?.error ?? null,
    meta: envelope?.meta ?? null
  };

  return {
    ok: false,
    status,
    envelope,
    error:
      cause === undefined
        ? baseError
        : {
            ...baseError,
            cause
          }
  };
}

function backendErrorMessage(error: ApiErrorBody): string {
  if (error.error_detail !== undefined && error.error_detail.trim().length > 0) {
    return `${error.error_message}: ${error.error_detail}`;
  }

  return error.error_message;
}

function isApiMeta(value: unknown): value is ApiMeta {
  if (!isRecord(value)) {
    return false;
  }

  return (
    typeof value["timestamp"] === "string" &&
    typeof value["processing_duration"] === "string"
  );
}

function isApiErrorBody(value: unknown): value is ApiErrorBody {
  if (!isRecord(value)) {
    return false;
  }

  const level = value["error_level"];
  if (!isApiErrorLevel(level)) {
    return false;
  }

  return (
    typeof value["error_code"] === "number" &&
    typeof value["error_message"] === "string" &&
    (value["error_detail"] === undefined || typeof value["error_detail"] === "string")
  );
}

function isApiErrorLevel(value: unknown): value is ApiErrorLevel {
  return (
    value === "ERROR" ||
    value === "WARN" ||
    value === "INFO" ||
    value === "DEBUG" ||
    value === "TRACE"
  );
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
