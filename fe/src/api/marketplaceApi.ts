import { requestApi } from "./client";
import type { ApiCallResult } from "./types";
import type {
  AdminOverviewResponse,
  BanListResponse,
  CreateBanRequest,
  CreateBannerRequest,
  CreateCentralBlogPostRequest,
  CreatePaymentIntentRequest,
  CreateProviderBlogPostRequest,
  MarketplaceCacheClearResponse,
  MarketplaceSearchReindexResponse,
  MarketplaceSearchResponse,
  MarketplaceSearchResultKind,
  ModerationDecisionRequest,
  PaymentIntentListResponse,
  PaymentIntentResponse,
  ProviderBlogPostResponse,
  ProviderDetailResponse,
  ProviderDirectoryResponse,
  ProviderProfileResponse,
  UpsertProviderProfileRequest,
  UpdateProviderBlogPostRequest,
  UpsertUserProfileRequest,
  UserProfileResponse
} from "./marketplaceTypes";

export function getProviderDirectory(
  query: Readonly<{ q?: string; subdivision_id?: number; subdivision_code?: string }>
): Promise<ApiCallResult<ProviderDirectoryResponse>> {
  const params: Record<string, string> = {};
  if (query.q !== undefined && query.q.length > 0) {
    params["q"] = query.q;
  }
  if (query.subdivision_id !== undefined) {
    params["subdivision_id"] = query.subdivision_id.toString();
  }
  if (query.subdivision_code !== undefined && query.subdivision_code.length > 0) {
    params["subdivision_code"] = query.subdivision_code;
  }

  return requestApi({
    method: "GET",
    path: "/api/v1/marketplace/providers",
    query: params
  });
}

export function searchMarketplace(
  query: Readonly<{ q: string; kind?: MarketplaceSearchResultKind; limit?: number }>
): Promise<ApiCallResult<MarketplaceSearchResponse>> {
  const params: Record<string, string> = { q: query.q };
  if (query.kind !== undefined) {
    params["kind"] = query.kind;
  }
  if (query.limit !== undefined) {
    params["limit"] = query.limit.toString();
  }

  return requestApi({
    method: "GET",
    path: "/api/v1/marketplace/search",
    query: params
  });
}

export function getProviderDetail(slug: string): Promise<ApiCallResult<ProviderDetailResponse>> {
  return requestApi({
    method: "GET",
    path: `/api/v1/marketplace/providers/${encodeURIComponent(slug)}`
  });
}

export function getUserMarketplaceProfile(): Promise<ApiCallResult<UserProfileResponse>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/marketplace/user/profile"
  });
}

export function upsertUserMarketplaceProfile(
  body: UpsertUserProfileRequest
): Promise<ApiCallResult<UserProfileResponse>> {
  return requestApi<UserProfileResponse, UpsertUserProfileRequest>({
    body,
    method: "POST",
    path: "/api/v1/marketplace/user/profile"
  });
}

export function getPaymentIntents(): Promise<ApiCallResult<PaymentIntentListResponse>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/marketplace/payments/intents"
  });
}

export function createPaymentIntent(
  body: CreatePaymentIntentRequest
): Promise<ApiCallResult<PaymentIntentResponse>> {
  return requestApi<PaymentIntentResponse, CreatePaymentIntentRequest>({
    body,
    method: "POST",
    path: "/api/v1/marketplace/payments/intents"
  });
}

export function getProviderDashboard(): Promise<ApiCallResult<ProviderDetailResponse>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/marketplace/provider/profile"
  });
}

export function upsertProviderMarketplaceProfile(
  body: UpsertProviderProfileRequest
): Promise<ApiCallResult<ProviderProfileResponse>> {
  return requestApi<ProviderProfileResponse, UpsertProviderProfileRequest>({
    body,
    method: "POST",
    path: "/api/v1/marketplace/provider/profile"
  });
}

export function createProviderBlogPost(
  body: CreateProviderBlogPostRequest
): Promise<ApiCallResult<ProviderBlogPostResponse>> {
  return requestApi<ProviderBlogPostResponse, CreateProviderBlogPostRequest>({
    body,
    method: "POST",
    path: "/api/v1/marketplace/provider/blog"
  });
}

export function updateProviderBlogPost(
  postId: string,
  body: UpdateProviderBlogPostRequest
): Promise<ApiCallResult<ProviderBlogPostResponse>> {
  return requestApi<ProviderBlogPostResponse, UpdateProviderBlogPostRequest>({
    body,
    method: "PUT",
    path: `/api/v1/marketplace/provider/blog/${encodeURIComponent(postId)}`
  });
}

export function moderateProviderProfile(
  providerProfileId: string,
  body: ModerationDecisionRequest
): Promise<ApiCallResult<ProviderProfileResponse>> {
  return requestApi<ProviderProfileResponse, ModerationDecisionRequest>({
    body,
    method: "POST",
    path: `/api/v1/marketplace/admin/providers/${encodeURIComponent(providerProfileId)}/moderation`
  });
}

export function moderateProviderBlogPost(
  providerBlogPostId: string,
  body: ModerationDecisionRequest
): Promise<ApiCallResult<ProviderBlogPostResponse>> {
  return requestApi<ProviderBlogPostResponse, ModerationDecisionRequest>({
    body,
    method: "POST",
    path: `/api/v1/marketplace/admin/provider-blog/${encodeURIComponent(providerBlogPostId)}/moderation`
  });
}

export function createCentralBlogPost(
  body: CreateCentralBlogPostRequest
): Promise<ApiCallResult<unknown>> {
  return requestApi<unknown, CreateCentralBlogPostRequest>({
    body,
    method: "POST",
    path: "/api/v1/marketplace/admin/blog"
  });
}

export function getAdminMarketplaceOverview(): Promise<ApiCallResult<AdminOverviewResponse>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/marketplace/admin/overview"
  });
}

export function reindexMarketplaceSearch(): Promise<
  ApiCallResult<MarketplaceSearchReindexResponse>
> {
  return requestApi({
    method: "POST",
    path: "/api/v1/marketplace/admin/search/reindex"
  });
}

export function clearMarketplacePublicCache(): Promise<
  ApiCallResult<MarketplaceCacheClearResponse>
> {
  return requestApi({
    method: "POST",
    path: "/api/v1/marketplace/admin/cache/clear"
  });
}

export function getActiveBans(): Promise<ApiCallResult<BanListResponse>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/marketplace/admin/bans/active"
  });
}

export function createBan(body: CreateBanRequest): Promise<ApiCallResult<unknown>> {
  return requestApi<unknown, CreateBanRequest>({
    body,
    method: "POST",
    path: "/api/v1/marketplace/admin/bans"
  });
}

export function createBanner(body: CreateBannerRequest): Promise<ApiCallResult<unknown>> {
  return requestApi<unknown, CreateBannerRequest>({
    body,
    method: "POST",
    path: "/api/v1/marketplace/admin/banners"
  });
}
