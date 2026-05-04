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
  PaymentIntentListResponse,
  PaymentIntentResponse,
  ProviderDetailResponse,
  ProviderDirectoryResponse,
  ProviderProfileResponse,
  UpsertProviderProfileRequest,
  UpsertUserProfileRequest,
  UserProfileResponse
} from "./marketplaceTypes";

export function getProviderDirectory(
  query: Readonly<{ q?: string; service_area?: string }>
): Promise<ApiCallResult<ProviderDirectoryResponse>> {
  return requestApi({
    method: "GET",
    path: "/api/v1/marketplace/providers",
    query
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
): Promise<ApiCallResult<unknown>> {
  return requestApi<unknown, CreateProviderBlogPostRequest>({
    body,
    method: "POST",
    path: "/api/v1/marketplace/provider/blog"
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
