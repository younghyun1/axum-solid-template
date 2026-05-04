import { requestApi } from "./client";
import type {
  ApiCallResult,
  CheckIfUserExistsRequest,
  CheckIfUserExistsResponse,
  CreateEmailVerificationQuestionAnswerRequest,
  CreateEmailVerificationQuestionRequest,
  DatabaseResetResponse,
  EmailVerificationChallengeResponse,
  EmailVerificationQuestionnaireResponse,
  HealthcheckResponse,
  LoginRequest,
  LoginResponse,
  LogoutResponse,
  MeResponse,
  RefreshSessionResponse,
  ReferenceCountryResponse,
  ReferenceLanguageResponse,
  ReferenceSubdivisionResponse,
  ResetPasswordProcessRequest,
  ResetPasswordRequest,
  ResetPasswordRequestResponse,
  ResetPasswordResponse,
  SignupRequest,
  SignupResponse,
  VerifyEmailChallengeRequest,
  VerifyEmailResponse
} from "./types";

export function getHealthcheck(): Promise<ApiCallResult<HealthcheckResponse>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/healthcheck"
  });
}

export function getCountries(): Promise<ApiCallResult<readonly ReferenceCountryResponse[]>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/reference/countries"
  });
}

export function getLanguages(): Promise<ApiCallResult<readonly ReferenceLanguageResponse[]>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/reference/languages"
  });
}

export function getCountrySubdivisions(
  countryCode: number
): Promise<ApiCallResult<readonly ReferenceSubdivisionResponse[]>> {
  return requestApi({
    method: "GET",
    path: `/api/v1/reference/countries/${countryCode.toString()}/subdivisions`
  });
}

export function checkIfUserExists(
  body: CheckIfUserExistsRequest
): Promise<ApiCallResult<CheckIfUserExistsResponse>> {
  return requestApi<CheckIfUserExistsResponse, CheckIfUserExistsRequest>({
    body,
    method: "POST",
    path: "/api/v1/auth/check-if-user-exists"
  });
}

export function signup(body: SignupRequest): Promise<ApiCallResult<SignupResponse>> {
  return requestApi<SignupResponse, SignupRequest>({
    body,
    method: "POST",
    path: "/api/v1/auth/signup"
  });
}

export function login(body: LoginRequest): Promise<ApiCallResult<LoginResponse>> {
  return requestApi<LoginResponse, LoginRequest>({
    body,
    method: "POST",
    path: "/api/v1/auth/login"
  });
}

export function refreshSession(): Promise<ApiCallResult<RefreshSessionResponse>> {
  return requestApi<RefreshSessionResponse>({
    method: "POST",
    path: "/api/v1/auth/refresh"
  });
}

export function me(): Promise<ApiCallResult<MeResponse>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/auth/me"
  });
}

export function logout(): Promise<ApiCallResult<LogoutResponse>> {
  return requestApi<LogoutResponse>({
    method: "POST",
    path: "/api/v1/auth/logout"
  });
}

export function requestPasswordReset(
  body: ResetPasswordRequest
): Promise<ApiCallResult<ResetPasswordRequestResponse>> {
  return requestApi<ResetPasswordRequestResponse, ResetPasswordRequest>({
    body,
    method: "POST",
    path: "/api/v1/auth/reset-password-request"
  });
}

export function resetPassword(
  body: ResetPasswordProcessRequest
): Promise<ApiCallResult<ResetPasswordResponse>> {
  return requestApi<ResetPasswordResponse, ResetPasswordProcessRequest>({
    body,
    method: "POST",
    path: "/api/v1/auth/reset-password"
  });
}

export function issueEmailVerificationChallenge(
  emailValidationTokenId: string
): Promise<ApiCallResult<EmailVerificationChallengeResponse>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/auth/email-verification/challenge",
    query: {
      email_validation_token_id: emailValidationTokenId
    }
  });
}

export function verifyEmail(
  body: VerifyEmailChallengeRequest
): Promise<ApiCallResult<VerifyEmailResponse>> {
  return requestApi<VerifyEmailResponse, VerifyEmailChallengeRequest>({
    body,
    method: "POST",
    path: "/api/v1/auth/verify-user-email"
  });
}

export function getEmailVerificationQuestions(): Promise<
  ApiCallResult<EmailVerificationQuestionnaireResponse>
> {
  return requestApi({
    method: "GET",
    path: "/api/v1/admin/email-verification/questions"
  });
}

export function createEmailVerificationQuestion(
  body: CreateEmailVerificationQuestionRequest
): Promise<ApiCallResult<EmailVerificationQuestionnaireResponse>> {
  return requestApi<EmailVerificationQuestionnaireResponse, CreateEmailVerificationQuestionRequest>({
    body,
    method: "POST",
    path: "/api/v1/admin/email-verification/questions"
  });
}

export function deleteEmailVerificationQuestion(
  questionId: string
): Promise<ApiCallResult<EmailVerificationQuestionnaireResponse>> {
  return requestApi({
    method: "DELETE",
    path: `/api/v1/admin/email-verification/questions/${questionId}`
  });
}

export function createEmailVerificationQuestionAnswer(
  questionId: string,
  body: CreateEmailVerificationQuestionAnswerRequest
): Promise<ApiCallResult<EmailVerificationQuestionnaireResponse>> {
  return requestApi<
    EmailVerificationQuestionnaireResponse,
    CreateEmailVerificationQuestionAnswerRequest
  >({
    body,
    method: "POST",
    path: `/api/v1/admin/email-verification/questions/${questionId}/answers`
  });
}

export function deleteEmailVerificationQuestionAnswer(
  questionId: string,
  answerId: string
): Promise<ApiCallResult<EmailVerificationQuestionnaireResponse>> {
  return requestApi({
    method: "DELETE",
    path: `/api/v1/admin/email-verification/questions/${questionId}/answers/${answerId}`
  });
}

export function resetDatabase(): Promise<ApiCallResult<DatabaseResetResponse>> {
  return requestApi<DatabaseResetResponse>({
    method: "POST",
    path: "/api/v1/admin/database/reset"
  });
}
