import {
  type BackendError,
  type BackendHealth,
  type BackendRequestOptions,
  type BackendResult,
  type BackendService,
  type HttpMethod,
} from "./types";

const DEFAULT_API_BASE_URL = "http://localhost:3000";

const resolveApiBaseUrl = (): string => {
  const configuredUrl = import.meta.env.VITE_API_BASE_URL;

  if (typeof configuredUrl === "string" && configuredUrl.length > 0) {
    return configuredUrl;
  }

  return DEFAULT_API_BASE_URL;
};

const createBackendError = (
  kind: BackendError["kind"],
  message: string,
  status?: number,
): BackendError => {
  if (status === undefined) {
    return {
      kind,
      message,
    };
  }

  return {
    kind,
    message,
    status,
  };
};

const readJson = async <T>(response: Response): Promise<BackendResult<T>> => {
  const contentType = response.headers.get("content-type");

  if (contentType === null || !contentType.includes("application/json")) {
    return {
      ok: false,
      error: createBackendError(
        "invalid-response",
        "Backend response did not contain JSON.",
        response.status,
      ),
    };
  }

  try {
    const body = (await response.json()) as T;

    return {
      ok: true,
      data: body,
    };
  } catch (error: unknown) {
    return {
      ok: false,
      error: createBackendError(
        "invalid-response",
        error instanceof Error ? error.message : "Backend JSON could not be decoded.",
        response.status,
      ),
    };
  }
};

const requestJson = async <TResponse>(
  method: HttpMethod,
  path: string,
  options: BackendRequestOptions = {},
): Promise<BackendResult<TResponse>> => {
  const url = new URL(path, resolveApiBaseUrl());
  const headers = new Headers(options.headers);

  headers.set("accept", "application/json");

  const body =
    options.body === undefined
      ? undefined
      : JSON.stringify(options.body);

  if (body !== undefined) {
    headers.set("content-type", "application/json");
  }

  let response: Response;

  try {
    response = await fetch(url, {
      body,
      credentials: options.credentials ?? "include",
      headers,
      method,
      signal: options.signal,
    });
  } catch (error: unknown) {
    return {
      ok: false,
      error: createBackendError(
        "network",
        error instanceof Error ? error.message : "Backend request failed.",
      ),
    };
  }

  if (!response.ok) {
    return {
      ok: false,
      error: createBackendError("http", response.statusText, response.status),
    };
  }

  return readJson<TResponse>(response);
};

export const createBackendService = (): BackendService => {
  return {
    health: () => requestJson<BackendHealth>("GET", "/health"),
    getJson: <TResponse>(path: string, options?: BackendRequestOptions) =>
      requestJson<TResponse>("GET", path, options),
    postJson: <TBody, TResponse>(
      path: string,
      body: TBody,
      options?: Omit<BackendRequestOptions, "body">,
    ) => requestJson<TResponse>("POST", path, { ...options, body }),
  };
};

export const backendService = createBackendService();
