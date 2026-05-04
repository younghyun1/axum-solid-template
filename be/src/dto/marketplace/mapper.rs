use crate::{
    domain::marketplace::{
        media::Image,
        moderation::{AdvertisementBanner, CentralBlogPost, ModerationBan},
        payments::{PaymentIntent, PaymentTransaction},
        provider::{ProviderBlogPost, ProviderProfile, UserProfileExtension},
    },
    dto::marketplace::response::{
        BanResponse, BannerResponse, CentralBlogPostResponse, ImageResponse, PaymentIntentResponse,
        PaymentTransactionResponse, ProviderBlogPostResponse, ProviderProfileResponse,
        UserProfileResponse,
    },
};

impl From<Image> for ImageResponse {
    fn from(image: Image) -> Self {
        Self {
            image_id: image.image_id,
            image_type: image.image_type,
            upload_status: image.image_upload_status,
            visibility: image.image_visibility,
            bucket: image.image_bucket,
            object_key: image.image_object_key,
            public_url: image.image_public_url,
            mime_type: image.image_mime_type,
            byte_size: image.image_byte_size,
            width: image.image_width,
            height: image.image_height,
            created_at: image.image_created_at,
            uploaded_at: image.image_uploaded_at,
        }
    }
}

impl From<UserProfileExtension> for UserProfileResponse {
    fn from(profile: UserProfileExtension) -> Self {
        Self {
            user_profile_extension_id: profile.user_profile_extension_id,
            user_id: profile.user_id,
            display_name: profile.user_profile_extension_display_name,
            bio: profile.user_profile_extension_bio,
            phone: profile.user_profile_extension_phone,
            public_email: profile.user_profile_extension_public_email,
            updated_at: profile.user_profile_extension_updated_at,
        }
    }
}

impl From<ProviderProfile> for ProviderProfileResponse {
    fn from(profile: ProviderProfile) -> Self {
        Self {
            provider_profile_id: profile.provider_profile_id,
            user_id: profile.user_id,
            slug: profile.provider_profile_slug,
            display_name: profile.provider_profile_display_name,
            headline: profile.provider_profile_headline,
            bio: profile.provider_profile_bio,
            service_area: profile.provider_profile_service_area,
            status: profile.provider_profile_status,
            moderation_status: profile.provider_profile_moderation_status,
            primary_image_id: profile.provider_profile_primary_image_id,
            created_at: profile.provider_profile_created_at,
            updated_at: profile.provider_profile_updated_at,
        }
    }
}

impl From<ProviderBlogPost> for ProviderBlogPostResponse {
    fn from(post: ProviderBlogPost) -> Self {
        Self {
            provider_blog_post_id: post.provider_blog_post_id,
            provider_profile_id: post.provider_profile_id,
            slug: post.provider_blog_post_slug,
            title: post.provider_blog_post_title,
            excerpt: post.provider_blog_post_excerpt,
            body: Some(post.provider_blog_post_body),
            status: post.provider_blog_post_status,
            moderation_status: post.provider_blog_post_moderation_status,
            hero_image_id: post.provider_blog_post_hero_image_id,
            published_at: post.provider_blog_post_published_at,
            created_at: post.provider_blog_post_created_at,
            updated_at: post.provider_blog_post_updated_at,
        }
    }
}

impl From<PaymentIntent> for PaymentIntentResponse {
    fn from(intent: PaymentIntent) -> Self {
        Self {
            payment_intent_id: intent.payment_intent_id,
            user_id: intent.user_id,
            provider_profile_id: intent.provider_profile_id,
            amount_minor_units: intent.payment_intent_amount_minor_units,
            currency_code: intent.payment_intent_currency,
            payment_provider: intent.payment_provider,
            status: intent.payment_intent_status,
            created_at: intent.payment_intent_created_at,
            updated_at: intent.payment_intent_updated_at,
        }
    }
}

impl From<PaymentTransaction> for PaymentTransactionResponse {
    fn from(transaction: PaymentTransaction) -> Self {
        Self {
            payment_transaction_id: transaction.payment_transaction_id,
            payment_intent_id: transaction.payment_intent_id,
            kind: transaction.payment_transaction_kind,
            status: transaction.payment_transaction_status,
            amount_minor_units: transaction.payment_transaction_amount_minor_units,
            currency_code: transaction.payment_transaction_currency,
            created_at: transaction.payment_transaction_created_at,
        }
    }
}

impl From<ModerationBan> for BanResponse {
    fn from(ban: ModerationBan) -> Self {
        Self {
            moderation_ban_id: ban.moderation_ban_id,
            target_user_id: ban.target_user_id,
            actor_user_id: ban.actor_user_id,
            scope: ban.moderation_ban_scope,
            reason: ban.moderation_ban_reason,
            starts_at: ban.moderation_ban_starts_at,
            expires_at: ban.moderation_ban_expires_at,
            revoked_at: ban.moderation_ban_revoked_at,
        }
    }
}

impl From<CentralBlogPost> for CentralBlogPostResponse {
    fn from(post: CentralBlogPost) -> Self {
        Self {
            central_blog_post_id: post.central_blog_post_id,
            author_user_id: post.author_user_id,
            slug: post.central_blog_post_slug,
            title: post.central_blog_post_title,
            excerpt: post.central_blog_post_excerpt,
            body: post.central_blog_post_body,
            status: post.central_blog_post_status,
            moderation_status: post.central_blog_post_moderation_status,
            published_at: post.central_blog_post_published_at,
        }
    }
}

impl From<AdvertisementBanner> for BannerResponse {
    fn from(banner: AdvertisementBanner) -> Self {
        Self {
            advertisement_banner_id: banner.advertisement_banner_id,
            placement: banner.advertisement_banner_placement,
            status: banner.advertisement_banner_status,
            title: banner.advertisement_banner_title,
            target_url: banner.advertisement_banner_target_url,
            priority: banner.advertisement_banner_priority,
            starts_at: banner.advertisement_banner_starts_at,
            ends_at: banner.advertisement_banner_ends_at,
            image_id: banner.advertisement_banner_image_id,
        }
    }
}
