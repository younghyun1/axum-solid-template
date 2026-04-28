import type {
  ApiMethod,
  CheckIfUserExistsRequest,
  JsonObject,
  LoginRequest,
  ResetPasswordProcessRequest,
  ResetPasswordRequest,
  SignupRequest
} from "./types";
import {
  field,
  optionalInteger,
  requiredInteger,
  requiredValue,
  type DemoField,
  type FormValues
} from "./forms";

export type AuthRequirement = "none" | "optional" | "required";

export interface BackendEndpoint {
  readonly id: string;
  readonly title: string;
  readonly method: ApiMethod;
  readonly path: string;
  readonly auth: AuthRequirement;
}

export interface DemoRequest {
  readonly method: ApiMethod;
  readonly path: string;
  readonly body?: JsonObject;
  readonly query?: Readonly<Record<string, string>>;
}

export type DemoBuildResult =
  | {
      readonly ok: true;
      readonly request: DemoRequest;
    }
  | {
      readonly ok: false;
      readonly message: string;
    };

export interface ApiDemo {
  readonly endpoint: BackendEndpoint;
  readonly fields: readonly DemoField[];
  readonly buildRequest: (values: FormValues) => DemoBuildResult;
}

export const backendApi = {
  healthcheck: endpoint("healthcheck", "Healthcheck", "GET", "/api/v1/healthcheck", "none"),
  checkUser: endpoint(
    "check-user",
    "Check User Email",
    "POST",
    "/api/v1/auth/check-if-user-exists",
    "none"
  ),
  signup: endpoint("signup", "Signup", "POST", "/api/v1/auth/signup", "none"),
  login: endpoint("login", "Login", "POST", "/api/v1/auth/login", "none"),
  me: endpoint("me", "Current User", "GET", "/api/v1/auth/me", "required"),
  logout: endpoint("logout", "Logout", "POST", "/api/v1/auth/logout", "required"),
  publicUser: endpoint("public-user", "Public User Info", "GET", "/api/v1/users/{user_name}", "optional"),
  resetPasswordRequest: endpoint(
    "reset-password-request",
    "Request Password Reset",
    "POST",
    "/api/v1/auth/reset-password-request",
    "none"
  ),
  resetPassword: endpoint(
    "reset-password",
    "Reset Password",
    "POST",
    "/api/v1/auth/reset-password",
    "none"
  ),
  verifyEmail: endpoint(
    "verify-email",
    "Verify Email",
    "GET",
    "/api/v1/auth/verify-user-email",
    "none"
  )
} as const;

export const apiDemos: readonly ApiDemo[] = [
  {
    endpoint: backendApi.healthcheck,
    fields: [],
    buildRequest: () => okRequest(endpointRequest(backendApi.healthcheck))
  },
  {
    endpoint: backendApi.checkUser,
    fields: [field("user_email", "Email", "email")],
    buildRequest: (values) => {
      const userEmail = requiredValue(values, "user_email", "Email");
      if (!userEmail.ok) {
        return userEmail;
      }

      const body: CheckIfUserExistsRequest = {
        user_email: userEmail.value
      };
      return okRequest(endpointRequest(backendApi.checkUser, body));
    }
  },
  {
    endpoint: backendApi.signup,
    fields: [
      field("user_name", "Username", "text"),
      field("user_email", "Email", "email"),
      field("user_password", "Password", "password"),
      field("user_country", "Country ID", "number", "1"),
      field("user_language", "Language ID", "number", "1"),
      field("user_subdivision", "Subdivision ID", "number", "", true)
    ],
    buildRequest: buildSignupRequest
  },
  {
    endpoint: backendApi.login,
    fields: [field("user_email", "Email", "email"), field("user_password", "Password", "password")],
    buildRequest: (values) => {
      const userEmail = requiredValue(values, "user_email", "Email");
      if (!userEmail.ok) {
        return userEmail;
      }
      const userPassword = requiredValue(values, "user_password", "Password");
      if (!userPassword.ok) {
        return userPassword;
      }

      const body: LoginRequest = {
        user_email: userEmail.value,
        user_password: userPassword.value
      };
      return okRequest(endpointRequest(backendApi.login, body));
    }
  },
  {
    endpoint: backendApi.me,
    fields: [],
    buildRequest: () => okRequest(endpointRequest(backendApi.me))
  },
  {
    endpoint: backendApi.logout,
    fields: [],
    buildRequest: () => okRequest(endpointRequest(backendApi.logout))
  },
  {
    endpoint: backendApi.publicUser,
    fields: [field("user_name", "Username", "text")],
    buildRequest: (values) => {
      const userName = requiredValue(values, "user_name", "Username");
      if (!userName.ok) {
        return userName;
      }
      return okRequest({
        method: backendApi.publicUser.method,
        path: `/api/v1/users/${encodeURIComponent(userName.value)}`
      });
    }
  },
  {
    endpoint: backendApi.resetPasswordRequest,
    fields: [field("user_email", "Email", "email")],
    buildRequest: (values) => {
      const userEmail = requiredValue(values, "user_email", "Email");
      if (!userEmail.ok) {
        return userEmail;
      }

      const body: ResetPasswordRequest = {
        user_email: userEmail.value
      };
      return okRequest(endpointRequest(backendApi.resetPasswordRequest, body));
    }
  },
  {
    endpoint: backendApi.resetPassword,
    fields: [
      field("password_reset_token", "Reset Token UUID", "text"),
      field("new_password", "New Password", "password")
    ],
    buildRequest: (values) => {
      const passwordResetToken = requiredValue(values, "password_reset_token", "Reset token");
      if (!passwordResetToken.ok) {
        return passwordResetToken;
      }
      const newPassword = requiredValue(values, "new_password", "New password");
      if (!newPassword.ok) {
        return newPassword;
      }

      const body: ResetPasswordProcessRequest = {
        password_reset_token: passwordResetToken.value,
        new_password: newPassword.value
      };
      return okRequest(endpointRequest(backendApi.resetPassword, body));
    }
  },
  {
    endpoint: backendApi.verifyEmail,
    fields: [field("email_validation_token_id", "Email Validation Token UUID", "text")],
    buildRequest: (values) => {
      const token = requiredValue(values, "email_validation_token_id", "Email validation token");
      if (!token.ok) {
        return token;
      }
      return okRequest({
        method: backendApi.verifyEmail.method,
        path: backendApi.verifyEmail.path,
        query: {
          email_validation_token_id: token.value
        }
      });
    }
  }
];

function buildSignupRequest(values: FormValues): DemoBuildResult {
  const userName = requiredValue(values, "user_name", "Username");
  if (!userName.ok) {
    return userName;
  }
  const userEmail = requiredValue(values, "user_email", "Email");
  if (!userEmail.ok) {
    return userEmail;
  }
  const userPassword = requiredValue(values, "user_password", "Password");
  if (!userPassword.ok) {
    return userPassword;
  }
  const userCountry = requiredInteger(values, "user_country", "Country ID");
  if (!userCountry.ok) {
    return userCountry;
  }
  const userLanguage = requiredInteger(values, "user_language", "Language ID");
  if (!userLanguage.ok) {
    return userLanguage;
  }
  const userSubdivision = optionalInteger(values, "user_subdivision", "Subdivision ID");
  if (!userSubdivision.ok) {
    return userSubdivision;
  }

  const body: SignupRequest = {
    user_name: userName.value,
    user_email: userEmail.value,
    user_password: userPassword.value,
    user_country: userCountry.value,
    user_language: userLanguage.value,
    user_subdivision: userSubdivision.value
  };

  return okRequest(endpointRequest(backendApi.signup, body));
}

function endpoint(
  id: string,
  title: string,
  method: ApiMethod,
  path: string,
  auth: AuthRequirement
): BackendEndpoint {
  return {
    id,
    title,
    method,
    path,
    auth
  };
}

function endpointRequest(endpoint: BackendEndpoint, body?: JsonObject): DemoRequest {
  const request: DemoRequest = {
    method: endpoint.method,
    path: endpoint.path
  };

  if (body !== undefined) {
    return {
      ...request,
      body
    };
  }

  return request;
}

function okRequest(request: DemoRequest): DemoBuildResult {
  return {
    ok: true,
    request
  };
}
