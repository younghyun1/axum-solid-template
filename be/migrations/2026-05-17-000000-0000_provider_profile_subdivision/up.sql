ALTER TABLE public.provider_profiles
    ADD COLUMN provider_profile_subdivision_id INTEGER,
    ADD CONSTRAINT fk_provider_profiles_subdivision
        FOREIGN KEY (provider_profile_subdivision_id)
        REFERENCES public.iso_country_subdivision (subdivision_id)
        ON DELETE SET NULL;

DROP INDEX IF EXISTS public.idx_provider_profiles_service_area;

ALTER TABLE public.provider_profiles
    DROP COLUMN provider_profile_service_area;

CREATE INDEX idx_provider_profiles_subdivision
    ON public.provider_profiles (provider_profile_subdivision_id);
