DROP TABLE IF EXISTS public.payment_processor_events;
DROP TABLE IF EXISTS public.payment_transactions;
DROP TABLE IF EXISTS public.payment_intents;
DROP TABLE IF EXISTS public.moderation_bans;

ALTER TABLE IF EXISTS public.advertisement_banners
    DROP CONSTRAINT IF EXISTS fk_advertisement_banners_image,
    DROP COLUMN IF EXISTS advertisement_banner_image_id;

ALTER TABLE IF EXISTS public.central_blog_posts
    DROP CONSTRAINT IF EXISTS fk_central_blog_posts_hero_image,
    DROP COLUMN IF EXISTS central_blog_post_hero_image_id;

ALTER TABLE IF EXISTS public.provider_blog_posts
    DROP CONSTRAINT IF EXISTS fk_provider_blog_posts_hero_image,
    DROP COLUMN IF EXISTS provider_blog_post_hero_image_id;

ALTER TABLE IF EXISTS public.provider_profiles
    DROP CONSTRAINT IF EXISTS fk_provider_profiles_primary_image,
    DROP COLUMN IF EXISTS provider_profile_primary_image_id;

DROP TABLE IF EXISTS public.images;
DROP TABLE IF EXISTS public.advertisement_banners;
DROP TABLE IF EXISTS public.central_blog_posts;
DROP TABLE IF EXISTS public.provider_blog_posts;
DROP TABLE IF EXISTS public.provider_profiles;
DROP TABLE IF EXISTS public.user_profile_extensions;

DROP TYPE IF EXISTS public.banner_status;
DROP TYPE IF EXISTS public.banner_placement;
DROP TYPE IF EXISTS public.processor_event_status;
DROP TYPE IF EXISTS public.payment_transaction_status;
DROP TYPE IF EXISTS public.payment_transaction_kind;
DROP TYPE IF EXISTS public.payment_intent_status;
DROP TYPE IF EXISTS public.payment_provider;
DROP TYPE IF EXISTS public.ban_scope;
DROP TYPE IF EXISTS public.blog_post_status;
DROP TYPE IF EXISTS public.moderation_status;
DROP TYPE IF EXISTS public.provider_profile_status;
DROP TYPE IF EXISTS public.image_visibility;
DROP TYPE IF EXISTS public.image_upload_status;
DROP TYPE IF EXISTS public.image_type;
