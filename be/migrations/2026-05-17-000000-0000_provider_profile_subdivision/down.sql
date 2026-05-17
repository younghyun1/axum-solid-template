DROP INDEX IF EXISTS public.idx_provider_profiles_subdivision;

ALTER TABLE public.provider_profiles
    ADD COLUMN provider_profile_service_area TEXT;

ALTER TABLE public.provider_profiles
    DROP CONSTRAINT IF EXISTS fk_provider_profiles_subdivision,
    DROP COLUMN IF EXISTS provider_profile_subdivision_id;

CREATE INDEX idx_provider_profiles_service_area
    ON public.provider_profiles (provider_profile_service_area);
