import type { JsonObject } from "./types";

export type ImageType =
  | "user_profile"
  | "provider_profile"
  | "provider_blog"
  | "central_blog"
  | "advertisement_banner";

export type ImageUploadStatus = "pending" | "uploaded" | "failed";
export type ImageVisibility = "private" | "public" | "hidden";
export type ProviderProfileStatus = "draft" | "published" | "suspended";
export type ModerationStatus = "pending" | "approved" | "rejected";
export type BlogPostStatus = "draft" | "published" | "archived";
export type BanScope = "account" | "provider" | "content";
export type PaymentProvider = "manual" | "external";
export type PaymentIntentStatus =
  | "created"
  | "requires_action"
  | "authorized"
  | "captured"
  | "cancelled"
  | "failed"
  | "refunded";
export type BannerPlacement = "homepage_top" | "directory_sidebar" | "provider_profile";
export type BannerStatus = "draft" | "active" | "paused" | "archived";
export type MarketplaceSearchResultKind = "provider" | "provider_blog" | "central_blog";

export interface ImageResponse {
  readonly image_id: string;
  readonly image_type: ImageType;
  readonly upload_status: ImageUploadStatus;
  readonly visibility: ImageVisibility;
  readonly bucket: string;
  readonly object_key: string;
  readonly public_url: string | null;
  readonly mime_type: string;
  readonly byte_size: number;
  readonly width: number | null;
  readonly height: number | null;
  readonly created_at: string;
  readonly uploaded_at: string | null;
}

export interface ProviderProfileResponse {
  readonly provider_profile_id: string;
  readonly user_id: string;
  readonly slug: string;
  readonly display_name: string;
  readonly headline: string | null;
  readonly bio: string | null;
  readonly service_area: string | null;
  readonly status: ProviderProfileStatus;
  readonly moderation_status: ModerationStatus;
  readonly primary_image_id: string | null;
  readonly created_at: string;
  readonly updated_at: string;
}

export interface ProviderDirectoryCardResponse {
  readonly provider_profile_id: string;
  readonly slug: string;
  readonly display_name: string;
  readonly headline: string | null;
  readonly service_area: string | null;
  readonly primary_image: ImageResponse | null;
}

export interface ProviderDirectoryResponse {
  readonly providers: readonly ProviderDirectoryCardResponse[];
}

export interface MarketplaceSearchResultResponse {
  readonly kind: MarketplaceSearchResultKind;
  readonly title: string;
  readonly subtitle: string;
  readonly slug: string;
  readonly url_path: string;
  readonly snippet: string;
  readonly score: number;
  readonly updated_at: string | null;
}

export interface MarketplaceSearchResponse {
  readonly results: readonly MarketplaceSearchResultResponse[];
}

export interface MarketplaceSearchReindexResponse {
  readonly indexed_documents: number;
}

export interface MarketplaceCacheClearResponse {
  readonly cleared: boolean;
}

export interface ProviderBlogPostResponse {
  readonly provider_blog_post_id: string;
  readonly provider_profile_id: string;
  readonly slug: string;
  readonly title: string;
  readonly excerpt: string | null;
  readonly body: string | null;
  readonly rendered_html: string | null;
  readonly status: BlogPostStatus;
  readonly moderation_status: ModerationStatus;
  readonly hero_image_id: string | null;
  readonly published_at: string | null;
  readonly created_at: string;
  readonly updated_at: string;
}

export interface ProviderDetailResponse {
  readonly profile: ProviderProfileResponse;
  readonly images: readonly ImageResponse[];
  readonly blog_posts: readonly ProviderBlogPostResponse[];
}

export interface UserProfileResponse {
  readonly user_profile_extension_id: string;
  readonly user_id: string;
  readonly display_name: string | null;
  readonly bio: string | null;
  readonly phone: string | null;
  readonly public_email: string | null;
  readonly updated_at: string;
}

export interface UpsertUserProfileRequest extends JsonObject {
  readonly display_name: string | null;
  readonly bio: string | null;
  readonly phone: string | null;
  readonly public_email: string | null;
}

export interface UpsertProviderProfileRequest extends JsonObject {
  readonly slug: string | null;
  readonly display_name: string;
  readonly headline: string | null;
  readonly bio: string | null;
  readonly service_area: string | null;
  readonly status: ProviderProfileStatus;
}

export interface CreateProviderBlogPostRequest extends JsonObject {
  readonly slug: string | null;
  readonly title: string;
  readonly excerpt: string | null;
  readonly body: string;
  readonly status: BlogPostStatus;
}

export interface CreateCentralBlogPostRequest extends JsonObject {
  readonly slug: string | null;
  readonly title: string;
  readonly excerpt: string | null;
  readonly body: string;
  readonly status: BlogPostStatus;
}

export interface CreatePaymentIntentRequest extends JsonObject {
  readonly provider_profile_id: string;
  readonly amount_minor_units: number;
  readonly currency_code: number;
  readonly payment_provider: PaymentProvider;
}

export interface PaymentIntentResponse {
  readonly payment_intent_id: string;
  readonly user_id: string;
  readonly provider_profile_id: string;
  readonly amount_minor_units: number;
  readonly currency_code: number;
  readonly payment_provider: PaymentProvider;
  readonly status: PaymentIntentStatus;
  readonly created_at: string;
  readonly updated_at: string;
}

export interface PaymentIntentListResponse {
  readonly payment_intents: readonly PaymentIntentResponse[];
}

export interface AdminOverviewResponse {
  readonly provider_count: number;
  readonly active_ban_count: number;
  readonly payment_intent_count: number;
  readonly active_banner_count: number;
}

export interface BanResponse {
  readonly moderation_ban_id: string;
  readonly target_user_id: string;
  readonly actor_user_id: string;
  readonly scope: BanScope;
  readonly reason: string;
  readonly starts_at: string;
  readonly expires_at: string | null;
  readonly revoked_at: string | null;
}

export interface BanListResponse {
  readonly bans: readonly BanResponse[];
}

export interface CreateBanRequest extends JsonObject {
  readonly target_user_id: string;
  readonly scope: BanScope;
  readonly reason: string;
  readonly starts_at: string | null;
  readonly expires_at: string | null;
}

export interface CreateBannerRequest extends JsonObject {
  readonly placement: BannerPlacement;
  readonly status: BannerStatus;
  readonly title: string;
  readonly target_url: string;
  readonly priority: number;
  readonly starts_at: string;
  readonly ends_at: string | null;
}
