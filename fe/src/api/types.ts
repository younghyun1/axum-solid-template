export type ApiMethod = "GET" | "POST";

export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | JsonObject | JsonValue[];

export interface JsonObject {
  readonly [key: string]: JsonValue;
}

export interface ApiMeta {
  readonly timestamp: string;
  readonly processing_duration: string;
  readonly details?: unknown;
}

export type ApiErrorLevel = "ERROR" | "WARN" | "INFO" | "DEBUG" | "TRACE";

export interface ApiErrorBody {
  readonly error_code: number;
  readonly error_level: ApiErrorLevel;
  readonly error_message: string;
  readonly error_detail?: string;
}

export interface ApiEnvelope<TData> {
  readonly success: boolean;
  readonly data: TData | null;
  readonly error: ApiErrorBody | null;
  readonly meta: ApiMeta;
}

export type ApiFailureKind = "backend" | "client" | "invalid_response" | "network" | "http";

export interface NormalizedApiError {
  readonly kind: ApiFailureKind;
  readonly message: string;
  readonly status: number | null;
  readonly backendError: ApiErrorBody | null;
  readonly meta: ApiMeta | null;
  readonly cause?: unknown;
}

export type ApiCallResult<TData> =
  | {
      readonly ok: true;
      readonly status: number;
      readonly envelope: ApiEnvelope<TData>;
      readonly data: TData | null;
      readonly meta: ApiMeta;
    }
  | {
      readonly ok: false;
      readonly status: number | null;
      readonly envelope: ApiEnvelope<unknown> | null;
      readonly error: NormalizedApiError;
    };

export interface HealthcheckResponse {
  readonly accepting_traffic: boolean;
}

export interface SignupRequest extends JsonObject {
  readonly user_name: string;
  readonly user_email: string;
  readonly user_password: string;
  readonly user_country: number;
  readonly user_language: number;
  readonly user_subdivision: number | null;
}

export interface LoginRequest extends JsonObject {
  readonly user_email: string;
  readonly user_password: string;
}

export interface CheckIfUserExistsRequest extends JsonObject {
  readonly user_email: string;
}

export interface ResetPasswordRequest extends JsonObject {
  readonly user_email: string;
}

export interface ResetPasswordProcessRequest extends JsonObject {
  readonly password_reset_token: string;
  readonly new_password: string;
}

export interface SignupResponse {
  readonly user_id: string;
  readonly user_name: string;
  readonly user_email: string;
  readonly verify_by: string;
}

export interface LoginResponse {
  readonly access_token: string;
  readonly token_type: string;
  readonly expires_at: string;
  readonly claims: Record<string, unknown>;
}

export interface MeResponse {
  readonly user_info: Record<string, unknown>;
  readonly claims: Record<string, unknown>;
}
