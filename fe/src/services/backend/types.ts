export type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export interface BackendRequestOptions {
  body?: unknown;
  credentials?: RequestCredentials;
  headers?: HeadersInit;
  signal?: AbortSignal;
}

export interface BackendHealth {
  status: string;
  service?: string;
  version?: string;
}

export type BackendErrorKind = "network" | "http" | "invalid-response";

export interface BackendError {
  kind: BackendErrorKind;
  message: string;
  status?: number;
}

export type BackendResult<T> =
  | {
      ok: true;
      data: T;
    }
  | {
      ok: false;
      error: BackendError;
    };

export interface BackendService {
  health: () => Promise<BackendResult<BackendHealth>>;
  getJson: <TResponse>(
    path: string,
    options?: BackendRequestOptions,
  ) => Promise<BackendResult<TResponse>>;
  postJson: <TBody, TResponse>(
    path: string,
    body: TBody,
    options?: Omit<BackendRequestOptions, "body">,
  ) => Promise<BackendResult<TResponse>>;
}

