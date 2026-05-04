export type ApiMethod = "DELETE" | "GET" | "POST";

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
  readonly user_role: SignupRoleType;
  readonly user_country: number;
  readonly user_language: number;
  readonly user_subdivision: number | null;
}

export type SignupRoleType = "user" | "service_provider";

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

export interface VerifyEmailChallengeRequest extends JsonObject {
  readonly email_validation_token_id: string;
  readonly email_verification_challenge_id: string;
  readonly email_verification_pow_nonce: string;
  readonly email_verification_answer: string;
  readonly email_verification_honeypot: string;
}

export interface CreateEmailVerificationQuestionRequest extends JsonObject {
  readonly email_verification_question_prompt: string;
  readonly email_verification_question_answers: string[];
}

export interface CreateEmailVerificationQuestionAnswerRequest extends JsonObject {
  readonly email_verification_question_answer_text: string;
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
  readonly claims: AccessTokenClaims;
}

export interface MeResponse {
  readonly user_info: UserInfo;
  readonly claims: AccessTokenClaims;
}

export type RoleType = "admin" | "moderator" | "service_provider" | "user" | "guest";

export interface AccessTokenClaims {
  readonly iss: string;
  readonly sub: string;
  readonly aud: readonly string[];
  readonly exp: number;
  readonly nbf: number;
  readonly iat: number;
  readonly jti: string;
  readonly token_type: "access";
  readonly user_id: string;
  readonly user_name: string;
  readonly user_email: string;
  readonly user_is_email_verified: boolean;
  readonly user_country: number;
  readonly user_language: number;
  readonly user_subdivision: number | null;
  readonly user_auth_token_version: number;
  readonly role_id: string;
  readonly role_name: string;
  readonly role_type: RoleType;
  readonly role_access_level: number;
  readonly issued_at_iso: string;
  readonly expires_at_iso: string;
}

export interface UserInfo {
  readonly user_id: string;
  readonly user_name: string;
  readonly user_email: string;
  readonly user_created_at: string;
  readonly user_updated_at: string;
  readonly user_last_login_at: string | null;
  readonly user_is_email_verified: boolean;
  readonly user_country: number;
  readonly user_language: number;
  readonly user_subdivision: number | null;
}

export interface LogoutResponse {
  readonly message: string;
}

export interface CheckIfUserExistsResponse {
  readonly email_exists: boolean;
}

export interface ResetPasswordRequestResponse {
  readonly user_email: string;
  readonly verify_by: string;
  readonly delivery_queued: boolean;
}

export interface ResetPasswordResponse {
  readonly user_id: string;
  readonly user_name: string;
  readonly user_email: string;
  readonly user_updated_at: string;
}

export interface VerifyEmailResponse {
  readonly user_id: string;
  readonly user_email: string;
  readonly verified_at: string;
}

export interface EmailVerificationChallengeResponse {
  readonly email_verification_challenge_id: string;
  readonly email_verification_question_id: string;
  readonly email_verification_question_prompt: string;
  readonly email_verification_pow_salt: string;
  readonly email_verification_pow_difficulty_bits: number;
  readonly email_verification_pow_algorithm: string;
  readonly email_verification_minimum_elapsed_ms: number;
  readonly email_verification_challenge_expires_at: string;
  readonly email_verification_questionnaire_revision: number;
}

export interface EmailVerificationQuestionnaireResponse {
  readonly email_verification_questionnaire_revision: number;
  readonly email_verification_questions: readonly EmailVerificationQuestion[];
}

export interface DatabaseResetResponse {
  readonly reverted_migration_count: number;
  readonly applied_migration_count: number;
}

export interface EmailVerificationQuestion {
  readonly email_verification_question_id: string;
  readonly email_verification_question_prompt: string;
  readonly email_verification_question_answers: readonly EmailVerificationQuestionAnswer[];
}

export interface EmailVerificationQuestionAnswer {
  readonly email_verification_question_answer_id: string;
  readonly email_verification_question_answer_text: string;
  readonly email_verification_question_answer_normalized: string;
}

export interface PublicUserInfoResponse {
  readonly user_id: string;
  readonly user_name: string;
  readonly user_created_at: string;
  readonly user_country: number;
}

export interface ReferenceCountryResponse {
  readonly country_code: number;
  readonly country_alpha2: string;
  readonly country_alpha3: string;
  readonly country_name: string;
  readonly country_primary_language: number;
  readonly country_currency: number;
  readonly phone_prefix: string;
  readonly country_flag: string;
  readonly is_country: boolean;
}

export interface ReferenceLanguageResponse {
  readonly language_code: number;
  readonly language_alpha2: string;
  readonly language_alpha3: string;
  readonly language_name: string;
}

export interface ReferenceSubdivisionResponse {
  readonly subdivision_id: number;
  readonly country_code: number;
  readonly subdivision_code: string;
  readonly subdivision_name: string;
  readonly subdivision_type: string | null;
  readonly country_flag: string;
}
