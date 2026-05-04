pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ban_scope"))]
    pub struct BanScope;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "banner_placement"))]
    pub struct BannerPlacement;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "banner_status"))]
    pub struct BannerStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "blog_post_status"))]
    pub struct BlogPostStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "image_type"))]
    pub struct ImageType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "image_upload_status"))]
    pub struct ImageUploadStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "image_visibility"))]
    pub struct ImageVisibility;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "job_run_status"))]
    pub struct JobRunStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "moderation_status"))]
    pub struct ModerationStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_intent_status"))]
    pub struct PaymentIntentStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_provider"))]
    pub struct PaymentProvider;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_transaction_kind"))]
    pub struct PaymentTransactionKind;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_transaction_status"))]
    pub struct PaymentTransactionStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "processor_event_status"))]
    pub struct ProcessorEventStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "provider_profile_status"))]
    pub struct ProviderProfileStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BannerPlacement;
    use super::sql_types::BannerStatus;

    advertisement_banners (advertisement_banner_id) {
        advertisement_banner_id -> Uuid,
        created_by_user_id -> Uuid,
        advertisement_banner_placement -> BannerPlacement,
        advertisement_banner_status -> BannerStatus,
        advertisement_banner_title -> Text,
        advertisement_banner_target_url -> Text,
        advertisement_banner_priority -> Int4,
        advertisement_banner_starts_at -> Timestamptz,
        advertisement_banner_ends_at -> Nullable<Timestamptz>,
        advertisement_banner_created_at -> Timestamptz,
        advertisement_banner_updated_at -> Timestamptz,
        advertisement_banner_image_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BlogPostStatus;
    use super::sql_types::ModerationStatus;

    central_blog_posts (central_blog_post_id) {
        central_blog_post_id -> Uuid,
        author_user_id -> Uuid,
        central_blog_post_slug -> Text,
        central_blog_post_title -> Text,
        central_blog_post_excerpt -> Nullable<Text>,
        central_blog_post_body -> Text,
        central_blog_post_status -> BlogPostStatus,
        central_blog_post_moderation_status -> ModerationStatus,
        central_blog_post_published_at -> Nullable<Timestamptz>,
        central_blog_post_created_at -> Timestamptz,
        central_blog_post_updated_at -> Timestamptz,
        central_blog_post_hero_image_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    auth_refresh_sessions (auth_refresh_session_id) {
        auth_refresh_session_id -> Uuid,
        user_id -> Uuid,
        auth_refresh_session_token_hash -> Varchar,
        auth_refresh_session_created_at -> Timestamptz,
        auth_refresh_session_expires_at -> Timestamptz,
        auth_refresh_session_last_used_at -> Nullable<Timestamptz>,
        auth_refresh_session_rotated_at -> Nullable<Timestamptz>,
        auth_refresh_session_revoked_at -> Nullable<Timestamptz>,
        auth_refresh_session_user_auth_token_version -> Int4,
    }
}

diesel::table! {
    email_verification_tokens (email_verification_token_id) {
        email_verification_token_id -> Uuid,
        user_id -> Uuid,
        email_verification_token -> Uuid,
        email_verification_token_expires_at -> Timestamptz,
        email_verification_token_created_at -> Timestamptz,
        email_verification_token_used_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ImageType;
    use super::sql_types::ImageUploadStatus;
    use super::sql_types::ImageVisibility;

    images (image_id) {
        image_id -> Uuid,
        image_type -> ImageType,
        image_upload_status -> ImageUploadStatus,
        image_visibility -> ImageVisibility,
        image_bucket -> Text,
        image_object_key -> Text,
        image_public_url -> Nullable<Text>,
        image_mime_type -> Text,
        image_byte_size -> Int8,
        image_width -> Nullable<Int4>,
        image_height -> Nullable<Int4>,
        image_checksum_sha256 -> Nullable<Text>,
        user_id -> Nullable<Uuid>,
        provider_profile_id -> Nullable<Uuid>,
        provider_blog_post_id -> Nullable<Uuid>,
        central_blog_post_id -> Nullable<Uuid>,
        advertisement_banner_id -> Nullable<Uuid>,
        image_created_at -> Timestamptz,
        image_updated_at -> Timestamptz,
        image_uploaded_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    iso_country (country_code) {
        country_code -> Int4,
        country_alpha2 -> Bpchar,
        country_alpha3 -> Bpchar,
        country_eng_name -> Varchar,
        country_primary_language -> Int4,
        country_currency -> Int4,
        phone_prefix -> Varchar,
        country_flag -> Bpchar,
        is_country -> Bool,
    }
}

diesel::table! {
    iso_country_subdivision (subdivision_id) {
        subdivision_id -> Int4,
        country_code -> Int4,
        subdivision_code -> Varchar,
        subdivision_name -> Varchar,
        subdivision_type -> Nullable<Varchar>,
    }
}

diesel::table! {
    iso_currency (currency_code) {
        currency_code -> Int4,
        currency_alpha3 -> Bpchar,
        currency_name -> Varchar,
    }
}

diesel::table! {
    iso_language (language_code) {
        language_code -> Int4,
        language_alpha2 -> Bpchar,
        language_alpha3 -> Bpchar,
        language_eng_name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::JobRunStatus;

    job_runs (job_run_id) {
        job_run_id -> Uuid,
        job_run_name -> Text,
        job_run_status -> JobRunStatus,
        job_run_scheduled_for -> Timestamptz,
        job_run_started_at -> Nullable<Timestamptz>,
        job_run_finished_at -> Nullable<Timestamptz>,
        job_run_duration_ms -> Nullable<Int8>,
        job_run_attempt -> Int4,
        job_run_error_code -> Nullable<Text>,
        job_run_error_message -> Nullable<Text>,
        job_run_metadata -> Jsonb,
        job_run_created_at -> Timestamptz,
        job_run_updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BanScope;

    moderation_bans (moderation_ban_id) {
        moderation_ban_id -> Uuid,
        target_user_id -> Uuid,
        actor_user_id -> Uuid,
        revoked_by_user_id -> Nullable<Uuid>,
        moderation_ban_scope -> BanScope,
        moderation_ban_reason -> Text,
        moderation_ban_starts_at -> Timestamptz,
        moderation_ban_expires_at -> Nullable<Timestamptz>,
        moderation_ban_revoked_at -> Nullable<Timestamptz>,
        moderation_ban_created_at -> Timestamptz,
    }
}

diesel::table! {
    password_reset_tokens (password_reset_token_id) {
        password_reset_token_id -> Uuid,
        user_id -> Uuid,
        password_reset_token -> Uuid,
        password_reset_token_expires_at -> Timestamptz,
        password_reset_token_created_at -> Timestamptz,
        password_reset_token_used_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PaymentIntentStatus;
    use super::sql_types::PaymentProvider;

    payment_intents (payment_intent_id) {
        payment_intent_id -> Uuid,
        user_id -> Uuid,
        provider_profile_id -> Uuid,
        payment_intent_amount_minor_units -> Int8,
        payment_intent_currency -> Int4,
        payment_provider -> PaymentProvider,
        payment_intent_status -> PaymentIntentStatus,
        payment_intent_processor_reference -> Nullable<Text>,
        payment_intent_metadata -> Jsonb,
        payment_intent_created_at -> Timestamptz,
        payment_intent_updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PaymentProvider;
    use super::sql_types::ProcessorEventStatus;

    payment_processor_events (payment_processor_event_id) {
        payment_processor_event_id -> Uuid,
        payment_provider -> PaymentProvider,
        payment_processor_event_external_id -> Text,
        payment_processor_event_status -> ProcessorEventStatus,
        payment_processor_event_payload -> Jsonb,
        payment_processor_event_processed_at -> Nullable<Timestamptz>,
        payment_processor_event_created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PaymentTransactionKind;
    use super::sql_types::PaymentTransactionStatus;

    payment_transactions (payment_transaction_id) {
        payment_transaction_id -> Uuid,
        payment_intent_id -> Uuid,
        payment_transaction_kind -> PaymentTransactionKind,
        payment_transaction_status -> PaymentTransactionStatus,
        payment_transaction_amount_minor_units -> Int8,
        payment_transaction_currency -> Int4,
        payment_transaction_processor_reference -> Nullable<Text>,
        payment_transaction_created_at -> Timestamptz,
    }
}

diesel::table! {
    permissions (permission_id) {
        permission_id -> Uuid,
        permission_name -> Text,
        permission_description -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BlogPostStatus;
    use super::sql_types::ModerationStatus;

    provider_blog_posts (provider_blog_post_id) {
        provider_blog_post_id -> Uuid,
        provider_profile_id -> Uuid,
        provider_blog_post_slug -> Text,
        provider_blog_post_title -> Text,
        provider_blog_post_excerpt -> Nullable<Text>,
        provider_blog_post_body -> Text,
        provider_blog_post_status -> BlogPostStatus,
        provider_blog_post_moderation_status -> ModerationStatus,
        provider_blog_post_published_at -> Nullable<Timestamptz>,
        provider_blog_post_created_at -> Timestamptz,
        provider_blog_post_updated_at -> Timestamptz,
        provider_blog_post_hero_image_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ModerationStatus;
    use super::sql_types::ProviderProfileStatus;

    provider_profiles (provider_profile_id) {
        provider_profile_id -> Uuid,
        user_id -> Uuid,
        provider_profile_slug -> Text,
        provider_profile_display_name -> Text,
        provider_profile_headline -> Nullable<Text>,
        provider_profile_bio -> Nullable<Text>,
        provider_profile_service_area -> Nullable<Text>,
        provider_profile_status -> ProviderProfileStatus,
        provider_profile_moderation_status -> ModerationStatus,
        provider_profile_created_at -> Timestamptz,
        provider_profile_updated_at -> Timestamptz,
        provider_profile_primary_image_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    role_permissions (role_permission_id) {
        role_permission_id -> Uuid,
        role_id -> Uuid,
        permission_id -> Uuid,
    }
}

diesel::table! {
    roles (role_id) {
        role_id -> Uuid,
        role_name -> Text,
        role_description -> Nullable<Text>,
    }
}

diesel::table! {
    user_roles (user_role_id) {
        user_role_id -> Uuid,
        user_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    user_profile_extensions (user_profile_extension_id) {
        user_profile_extension_id -> Uuid,
        user_id -> Uuid,
        user_profile_extension_display_name -> Nullable<Text>,
        user_profile_extension_bio -> Nullable<Text>,
        user_profile_extension_phone -> Nullable<Text>,
        user_profile_extension_public_email -> Nullable<Text>,
        user_profile_extension_created_at -> Timestamptz,
        user_profile_extension_updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        user_name -> Varchar,
        user_email -> Varchar,
        user_password_hash -> Varchar,
        user_created_at -> Timestamptz,
        user_updated_at -> Timestamptz,
        user_password_changed_at -> Timestamptz,
        user_last_login_at -> Nullable<Timestamptz>,
        user_is_email_verified -> Bool,
        user_auth_token_version -> Int4,
        user_country -> Int4,
        user_language -> Int4,
        user_subdivision -> Nullable<Int4>,
    }
}

diesel::joinable!(auth_refresh_sessions -> users (user_id));
diesel::joinable!(central_blog_posts -> users (author_user_id));
diesel::joinable!(email_verification_tokens -> users (user_id));
diesel::joinable!(images -> advertisement_banners (advertisement_banner_id));
diesel::joinable!(images -> central_blog_posts (central_blog_post_id));
diesel::joinable!(images -> provider_blog_posts (provider_blog_post_id));
diesel::joinable!(images -> provider_profiles (provider_profile_id));
diesel::joinable!(images -> users (user_id));
diesel::joinable!(iso_country -> iso_currency (country_currency));
diesel::joinable!(iso_country -> iso_language (country_primary_language));
diesel::joinable!(iso_country_subdivision -> iso_country (country_code));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(payment_intents -> iso_currency (payment_intent_currency));
diesel::joinable!(payment_intents -> provider_profiles (provider_profile_id));
diesel::joinable!(payment_intents -> users (user_id));
diesel::joinable!(payment_transactions -> iso_currency (payment_transaction_currency));
diesel::joinable!(payment_transactions -> payment_intents (payment_intent_id));
diesel::joinable!(provider_blog_posts -> provider_profiles (provider_profile_id));
diesel::joinable!(provider_profiles -> users (user_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(user_profile_extensions -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(users -> iso_country (user_country));
diesel::joinable!(users -> iso_country_subdivision (user_subdivision));
diesel::joinable!(users -> iso_language (user_language));

diesel::allow_tables_to_appear_in_same_query!(
    advertisement_banners,
    auth_refresh_sessions,
    central_blog_posts,
    email_verification_tokens,
    images,
    iso_country,
    iso_country_subdivision,
    iso_currency,
    iso_language,
    job_runs,
    moderation_bans,
    password_reset_tokens,
    payment_intents,
    payment_processor_events,
    payment_transactions,
    permissions,
    provider_blog_posts,
    provider_profiles,
    role_permissions,
    roles,
    user_profile_extensions,
    user_roles,
    users,
);
