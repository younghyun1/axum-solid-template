CREATE TYPE public.image_type AS ENUM (
    'user_profile',
    'provider_profile',
    'provider_blog',
    'central_blog',
    'advertisement_banner'
);

CREATE TYPE public.image_upload_status AS ENUM ('pending', 'uploaded', 'failed');
CREATE TYPE public.image_visibility AS ENUM ('private', 'public', 'hidden');
CREATE TYPE public.provider_profile_status AS ENUM ('draft', 'published', 'suspended');
CREATE TYPE public.moderation_status AS ENUM ('pending', 'approved', 'rejected');
CREATE TYPE public.blog_post_status AS ENUM ('draft', 'published', 'archived');
CREATE TYPE public.ban_scope AS ENUM ('account', 'provider', 'content');
CREATE TYPE public.payment_provider AS ENUM ('manual', 'external');
CREATE TYPE public.payment_intent_status AS ENUM (
    'created',
    'requires_action',
    'authorized',
    'captured',
    'cancelled',
    'failed',
    'refunded'
);
CREATE TYPE public.payment_transaction_kind AS ENUM ('authorization', 'capture', 'refund', 'adjustment');
CREATE TYPE public.payment_transaction_status AS ENUM ('pending', 'succeeded', 'failed');
CREATE TYPE public.processor_event_status AS ENUM ('pending', 'processed', 'failed');
CREATE TYPE public.banner_placement AS ENUM ('homepage_top', 'directory_sidebar', 'provider_profile');
CREATE TYPE public.banner_status AS ENUM ('draft', 'active', 'paused', 'archived');

CREATE TABLE public.user_profile_extensions (
    user_profile_extension_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL UNIQUE,
    user_profile_extension_display_name TEXT,
    user_profile_extension_bio TEXT,
    user_profile_extension_phone TEXT,
    user_profile_extension_public_email TEXT,
    user_profile_extension_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    user_profile_extension_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_user_profile_extensions_user
        FOREIGN KEY (user_id) REFERENCES public.users (user_id) ON DELETE CASCADE
);

CREATE TABLE public.provider_profiles (
    provider_profile_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL UNIQUE,
    provider_profile_slug TEXT NOT NULL UNIQUE,
    provider_profile_display_name TEXT NOT NULL,
    provider_profile_headline TEXT,
    provider_profile_bio TEXT,
    provider_profile_service_area TEXT,
    provider_profile_status public.provider_profile_status NOT NULL DEFAULT 'draft',
    provider_profile_moderation_status public.moderation_status NOT NULL DEFAULT 'pending',
    provider_profile_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    provider_profile_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_provider_profiles_user
        FOREIGN KEY (user_id) REFERENCES public.users (user_id) ON DELETE CASCADE,
    CONSTRAINT provider_profiles_slug_not_empty CHECK (length(trim(provider_profile_slug)) > 0),
    CONSTRAINT provider_profiles_display_name_not_empty CHECK (length(trim(provider_profile_display_name)) > 0)
);

CREATE TABLE public.provider_blog_posts (
    provider_blog_post_id UUID PRIMARY KEY DEFAULT uuidv7(),
    provider_profile_id UUID NOT NULL,
    provider_blog_post_slug TEXT NOT NULL,
    provider_blog_post_title TEXT NOT NULL,
    provider_blog_post_excerpt TEXT,
    provider_blog_post_body TEXT NOT NULL,
    provider_blog_post_status public.blog_post_status NOT NULL DEFAULT 'draft',
    provider_blog_post_moderation_status public.moderation_status NOT NULL DEFAULT 'pending',
    provider_blog_post_published_at TIMESTAMPTZ,
    provider_blog_post_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    provider_blog_post_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_provider_blog_posts_profile
        FOREIGN KEY (provider_profile_id) REFERENCES public.provider_profiles (provider_profile_id) ON DELETE CASCADE,
    CONSTRAINT provider_blog_posts_slug_not_empty CHECK (length(trim(provider_blog_post_slug)) > 0),
    CONSTRAINT provider_blog_posts_title_not_empty CHECK (length(trim(provider_blog_post_title)) > 0),
    CONSTRAINT provider_blog_posts_body_not_empty CHECK (length(trim(provider_blog_post_body)) > 0),
    UNIQUE (provider_profile_id, provider_blog_post_slug)
);

CREATE TABLE public.central_blog_posts (
    central_blog_post_id UUID PRIMARY KEY DEFAULT uuidv7(),
    author_user_id UUID NOT NULL,
    central_blog_post_slug TEXT NOT NULL UNIQUE,
    central_blog_post_title TEXT NOT NULL,
    central_blog_post_excerpt TEXT,
    central_blog_post_body TEXT NOT NULL,
    central_blog_post_status public.blog_post_status NOT NULL DEFAULT 'draft',
    central_blog_post_moderation_status public.moderation_status NOT NULL DEFAULT 'pending',
    central_blog_post_published_at TIMESTAMPTZ,
    central_blog_post_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    central_blog_post_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_central_blog_posts_author
        FOREIGN KEY (author_user_id) REFERENCES public.users (user_id) ON DELETE RESTRICT,
    CONSTRAINT central_blog_posts_slug_not_empty CHECK (length(trim(central_blog_post_slug)) > 0),
    CONSTRAINT central_blog_posts_title_not_empty CHECK (length(trim(central_blog_post_title)) > 0),
    CONSTRAINT central_blog_posts_body_not_empty CHECK (length(trim(central_blog_post_body)) > 0)
);

CREATE TABLE public.advertisement_banners (
    advertisement_banner_id UUID PRIMARY KEY DEFAULT uuidv7(),
    created_by_user_id UUID NOT NULL,
    advertisement_banner_placement public.banner_placement NOT NULL,
    advertisement_banner_status public.banner_status NOT NULL DEFAULT 'draft',
    advertisement_banner_title TEXT NOT NULL,
    advertisement_banner_target_url TEXT NOT NULL,
    advertisement_banner_priority INTEGER NOT NULL DEFAULT 0,
    advertisement_banner_starts_at TIMESTAMPTZ NOT NULL,
    advertisement_banner_ends_at TIMESTAMPTZ,
    advertisement_banner_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    advertisement_banner_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_advertisement_banners_created_by
        FOREIGN KEY (created_by_user_id) REFERENCES public.users (user_id) ON DELETE RESTRICT,
    CONSTRAINT advertisement_banners_title_not_empty CHECK (length(trim(advertisement_banner_title)) > 0),
    CONSTRAINT advertisement_banners_target_url_not_empty CHECK (length(trim(advertisement_banner_target_url)) > 0),
    CONSTRAINT advertisement_banners_schedule_valid CHECK (
        advertisement_banner_ends_at IS NULL
        OR advertisement_banner_ends_at > advertisement_banner_starts_at
    )
);

CREATE TABLE public.images (
    image_id UUID PRIMARY KEY DEFAULT uuidv7(),
    image_type public.image_type NOT NULL,
    image_upload_status public.image_upload_status NOT NULL DEFAULT 'pending',
    image_visibility public.image_visibility NOT NULL DEFAULT 'private',
    image_bucket TEXT NOT NULL,
    image_object_key TEXT NOT NULL,
    image_public_url TEXT,
    image_mime_type TEXT NOT NULL,
    image_byte_size BIGINT NOT NULL,
    image_width INTEGER,
    image_height INTEGER,
    image_checksum_sha256 TEXT,
    user_id UUID,
    provider_profile_id UUID,
    provider_blog_post_id UUID,
    central_blog_post_id UUID,
    advertisement_banner_id UUID,
    image_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    image_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    image_uploaded_at TIMESTAMPTZ,
    CONSTRAINT fk_images_user
        FOREIGN KEY (user_id) REFERENCES public.users (user_id) ON DELETE CASCADE,
    CONSTRAINT fk_images_provider_profile
        FOREIGN KEY (provider_profile_id) REFERENCES public.provider_profiles (provider_profile_id) ON DELETE CASCADE,
    CONSTRAINT fk_images_provider_blog_post
        FOREIGN KEY (provider_blog_post_id) REFERENCES public.provider_blog_posts (provider_blog_post_id) ON DELETE CASCADE,
    CONSTRAINT fk_images_central_blog_post
        FOREIGN KEY (central_blog_post_id) REFERENCES public.central_blog_posts (central_blog_post_id) ON DELETE CASCADE,
    CONSTRAINT fk_images_advertisement_banner
        FOREIGN KEY (advertisement_banner_id) REFERENCES public.advertisement_banners (advertisement_banner_id) ON DELETE CASCADE,
    CONSTRAINT images_bucket_not_empty CHECK (length(trim(image_bucket)) > 0),
    CONSTRAINT images_object_key_not_empty CHECK (length(trim(image_object_key)) > 0),
    CONSTRAINT images_mime_type_not_empty CHECK (length(trim(image_mime_type)) > 0),
    CONSTRAINT images_byte_size_positive CHECK (image_byte_size > 0),
    CONSTRAINT images_dimensions_positive CHECK (
        (image_width IS NULL OR image_width > 0)
        AND (image_height IS NULL OR image_height > 0)
    ),
    CONSTRAINT images_type_owner CHECK (
        (
            image_type = 'user_profile'
            AND user_id IS NOT NULL
            AND provider_profile_id IS NULL
            AND provider_blog_post_id IS NULL
            AND central_blog_post_id IS NULL
            AND advertisement_banner_id IS NULL
        )
        OR (
            image_type = 'provider_profile'
            AND user_id IS NULL
            AND provider_profile_id IS NOT NULL
            AND provider_blog_post_id IS NULL
            AND central_blog_post_id IS NULL
            AND advertisement_banner_id IS NULL
        )
        OR (
            image_type = 'provider_blog'
            AND user_id IS NULL
            AND provider_profile_id IS NULL
            AND provider_blog_post_id IS NOT NULL
            AND central_blog_post_id IS NULL
            AND advertisement_banner_id IS NULL
        )
        OR (
            image_type = 'central_blog'
            AND user_id IS NULL
            AND provider_profile_id IS NULL
            AND provider_blog_post_id IS NULL
            AND central_blog_post_id IS NOT NULL
            AND advertisement_banner_id IS NULL
        )
        OR (
            image_type = 'advertisement_banner'
            AND user_id IS NULL
            AND provider_profile_id IS NULL
            AND provider_blog_post_id IS NULL
            AND central_blog_post_id IS NULL
            AND advertisement_banner_id IS NOT NULL
        )
    )
);

ALTER TABLE public.provider_profiles
    ADD COLUMN provider_profile_primary_image_id UUID,
    ADD CONSTRAINT fk_provider_profiles_primary_image
        FOREIGN KEY (provider_profile_primary_image_id) REFERENCES public.images (image_id) ON DELETE SET NULL;

ALTER TABLE public.provider_blog_posts
    ADD COLUMN provider_blog_post_hero_image_id UUID,
    ADD CONSTRAINT fk_provider_blog_posts_hero_image
        FOREIGN KEY (provider_blog_post_hero_image_id) REFERENCES public.images (image_id) ON DELETE SET NULL;

ALTER TABLE public.central_blog_posts
    ADD COLUMN central_blog_post_hero_image_id UUID,
    ADD CONSTRAINT fk_central_blog_posts_hero_image
        FOREIGN KEY (central_blog_post_hero_image_id) REFERENCES public.images (image_id) ON DELETE SET NULL;

ALTER TABLE public.advertisement_banners
    ADD COLUMN advertisement_banner_image_id UUID,
    ADD CONSTRAINT fk_advertisement_banners_image
        FOREIGN KEY (advertisement_banner_image_id) REFERENCES public.images (image_id) ON DELETE SET NULL;

CREATE TABLE public.moderation_bans (
    moderation_ban_id UUID PRIMARY KEY DEFAULT uuidv7(),
    target_user_id UUID NOT NULL,
    actor_user_id UUID NOT NULL,
    revoked_by_user_id UUID,
    moderation_ban_scope public.ban_scope NOT NULL,
    moderation_ban_reason TEXT NOT NULL,
    moderation_ban_starts_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    moderation_ban_expires_at TIMESTAMPTZ,
    moderation_ban_revoked_at TIMESTAMPTZ,
    moderation_ban_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_moderation_bans_target
        FOREIGN KEY (target_user_id) REFERENCES public.users (user_id) ON DELETE CASCADE,
    CONSTRAINT fk_moderation_bans_actor
        FOREIGN KEY (actor_user_id) REFERENCES public.users (user_id) ON DELETE RESTRICT,
    CONSTRAINT fk_moderation_bans_revoked_by
        FOREIGN KEY (revoked_by_user_id) REFERENCES public.users (user_id) ON DELETE SET NULL,
    CONSTRAINT moderation_bans_reason_not_empty CHECK (length(trim(moderation_ban_reason)) > 0),
    CONSTRAINT moderation_bans_expires_after_start CHECK (
        moderation_ban_expires_at IS NULL
        OR moderation_ban_expires_at > moderation_ban_starts_at
    ),
    CONSTRAINT moderation_bans_revoked_by_consistent CHECK (
        (moderation_ban_revoked_at IS NULL AND revoked_by_user_id IS NULL)
        OR (moderation_ban_revoked_at IS NOT NULL AND revoked_by_user_id IS NOT NULL)
    )
);

CREATE TABLE public.payment_intents (
    payment_intent_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL,
    provider_profile_id UUID NOT NULL,
    payment_intent_amount_minor_units BIGINT NOT NULL,
    payment_intent_currency INTEGER NOT NULL,
    payment_provider public.payment_provider NOT NULL DEFAULT 'manual',
    payment_intent_status public.payment_intent_status NOT NULL DEFAULT 'created',
    payment_intent_processor_reference TEXT,
    payment_intent_metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    payment_intent_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    payment_intent_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_payment_intents_user
        FOREIGN KEY (user_id) REFERENCES public.users (user_id) ON DELETE RESTRICT,
    CONSTRAINT fk_payment_intents_provider_profile
        FOREIGN KEY (provider_profile_id) REFERENCES public.provider_profiles (provider_profile_id) ON DELETE RESTRICT,
    CONSTRAINT fk_payment_intents_currency
        FOREIGN KEY (payment_intent_currency) REFERENCES public.iso_currency (currency_code),
    CONSTRAINT payment_intents_amount_positive CHECK (payment_intent_amount_minor_units > 0)
);

CREATE TABLE public.payment_transactions (
    payment_transaction_id UUID PRIMARY KEY DEFAULT uuidv7(),
    payment_intent_id UUID NOT NULL,
    payment_transaction_kind public.payment_transaction_kind NOT NULL,
    payment_transaction_status public.payment_transaction_status NOT NULL,
    payment_transaction_amount_minor_units BIGINT NOT NULL,
    payment_transaction_currency INTEGER NOT NULL,
    payment_transaction_processor_reference TEXT,
    payment_transaction_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_payment_transactions_intent
        FOREIGN KEY (payment_intent_id) REFERENCES public.payment_intents (payment_intent_id) ON DELETE CASCADE,
    CONSTRAINT fk_payment_transactions_currency
        FOREIGN KEY (payment_transaction_currency) REFERENCES public.iso_currency (currency_code),
    CONSTRAINT payment_transactions_amount_non_zero CHECK (payment_transaction_amount_minor_units <> 0)
);

CREATE TABLE public.payment_processor_events (
    payment_processor_event_id UUID PRIMARY KEY DEFAULT uuidv7(),
    payment_provider public.payment_provider NOT NULL,
    payment_processor_event_external_id TEXT NOT NULL,
    payment_processor_event_status public.processor_event_status NOT NULL DEFAULT 'pending',
    payment_processor_event_payload JSONB NOT NULL,
    payment_processor_event_processed_at TIMESTAMPTZ,
    payment_processor_event_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT payment_processor_events_external_id_not_empty CHECK (length(trim(payment_processor_event_external_id)) > 0),
    UNIQUE (payment_provider, payment_processor_event_external_id)
);

CREATE INDEX idx_user_profile_extensions_user_id ON public.user_profile_extensions (user_id);

CREATE INDEX idx_provider_profiles_user_id ON public.provider_profiles (user_id);
CREATE INDEX idx_provider_profiles_public
    ON public.provider_profiles (provider_profile_status, provider_profile_moderation_status, provider_profile_slug);
CREATE INDEX idx_provider_profiles_service_area ON public.provider_profiles (provider_profile_service_area);

CREATE INDEX idx_provider_blog_posts_profile ON public.provider_blog_posts (provider_profile_id);
CREATE INDEX idx_provider_blog_posts_public
    ON public.provider_blog_posts (provider_blog_post_status, provider_blog_post_moderation_status, provider_blog_post_published_at);

CREATE INDEX idx_central_blog_posts_public
    ON public.central_blog_posts (central_blog_post_status, central_blog_post_moderation_status, central_blog_post_published_at);

CREATE INDEX idx_advertisement_banners_active
    ON public.advertisement_banners (advertisement_banner_placement, advertisement_banner_status, advertisement_banner_starts_at, advertisement_banner_ends_at);

CREATE INDEX idx_images_user ON public.images (user_id);
CREATE INDEX idx_images_provider_profile ON public.images (provider_profile_id);
CREATE INDEX idx_images_provider_blog_post ON public.images (provider_blog_post_id);
CREATE INDEX idx_images_central_blog_post ON public.images (central_blog_post_id);
CREATE INDEX idx_images_advertisement_banner ON public.images (advertisement_banner_id);
CREATE INDEX idx_images_visibility_type ON public.images (image_visibility, image_type);
CREATE INDEX idx_images_pending_created_at ON public.images (image_upload_status, image_created_at);

CREATE INDEX idx_moderation_bans_target_active
    ON public.moderation_bans (target_user_id, moderation_ban_revoked_at, moderation_ban_starts_at, moderation_ban_expires_at);
CREATE INDEX idx_moderation_bans_actor ON public.moderation_bans (actor_user_id);

CREATE INDEX idx_payment_intents_user ON public.payment_intents (user_id, payment_intent_created_at);
CREATE INDEX idx_payment_intents_provider ON public.payment_intents (provider_profile_id, payment_intent_created_at);
CREATE INDEX idx_payment_intents_status ON public.payment_intents (payment_intent_status, payment_intent_updated_at);
CREATE INDEX idx_payment_transactions_intent ON public.payment_transactions (payment_intent_id);
CREATE INDEX idx_payment_processor_events_status ON public.payment_processor_events (payment_processor_event_status, payment_processor_event_created_at);
