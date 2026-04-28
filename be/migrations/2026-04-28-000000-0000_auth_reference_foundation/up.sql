CREATE TABLE public.iso_language (
    language_code INTEGER PRIMARY KEY,
    language_alpha2 CHAR(2) NOT NULL,
    language_alpha3 CHAR(3) NOT NULL,
    language_eng_name VARCHAR(255) NOT NULL
);

CREATE UNIQUE INDEX idx_language_alpha2 ON public.iso_language (language_alpha2);
CREATE UNIQUE INDEX idx_language_alpha3 ON public.iso_language (language_alpha3);
CREATE INDEX idx_language_eng_name ON public.iso_language (language_eng_name);

CREATE TABLE public.iso_currency (
    currency_code INTEGER PRIMARY KEY,
    currency_alpha3 CHAR(3) NOT NULL,
    currency_name VARCHAR(255) NOT NULL
);

CREATE UNIQUE INDEX idx_currency_alpha3 ON public.iso_currency (currency_alpha3);

CREATE TABLE public.iso_country (
    country_code INTEGER PRIMARY KEY,
    country_alpha2 CHAR(2) NOT NULL,
    country_alpha3 CHAR(3) NOT NULL,
    country_eng_name VARCHAR(255) NOT NULL,
    country_primary_language CHAR(2),
    country_currency INTEGER,
    phone_prefix VARCHAR(10),
    CONSTRAINT iso_country_country_primary_language_fkey
        FOREIGN KEY (country_primary_language) REFERENCES public.iso_language (language_alpha2),
    CONSTRAINT iso_country_country_currency_fkey
        FOREIGN KEY (country_currency) REFERENCES public.iso_currency (currency_code)
);

CREATE UNIQUE INDEX idx_country_alpha2 ON public.iso_country (country_alpha2);
CREATE UNIQUE INDEX idx_country_alpha3 ON public.iso_country (country_alpha3);
CREATE INDEX idx_country_eng_name ON public.iso_country (country_eng_name);
CREATE INDEX idx_country_language ON public.iso_country (country_primary_language);
CREATE INDEX idx_country_currency ON public.iso_country (country_currency);
CREATE INDEX idx_phone_prefix ON public.iso_country (phone_prefix);

CREATE TABLE public.iso_country_subdivision (
    subdivision_id INTEGER PRIMARY KEY,
    country_code INTEGER NOT NULL,
    subdivision_code VARCHAR(10) NOT NULL,
    subdivision_name VARCHAR(255) NOT NULL,
    subdivision_type VARCHAR(50),
    CONSTRAINT iso_country_subdivision_country_code_fkey
        FOREIGN KEY (country_code) REFERENCES public.iso_country (country_code),
    UNIQUE (country_code, subdivision_code)
);

CREATE INDEX idx_subdivision_country ON public.iso_country_subdivision (country_code);

CREATE TABLE public.users (
    user_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_name VARCHAR NOT NULL,
    user_email VARCHAR NOT NULL,
    user_password_hash VARCHAR NOT NULL,
    user_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    user_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    user_password_changed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    user_last_login_at TIMESTAMPTZ,
    user_is_email_verified BOOLEAN NOT NULL DEFAULT false,
    user_auth_token_version INTEGER NOT NULL DEFAULT 0,
    user_country INTEGER NOT NULL,
    user_language INTEGER NOT NULL,
    user_subdivision INTEGER,
    CONSTRAINT fk_user_country FOREIGN KEY (user_country) REFERENCES public.iso_country(country_code),
    CONSTRAINT fk_user_language FOREIGN KEY (user_language) REFERENCES public.iso_language(language_code),
    CONSTRAINT fk_user_subdivision FOREIGN KEY (user_subdivision) REFERENCES public.iso_country_subdivision(subdivision_id)
);

CREATE UNIQUE INDEX idx_users_email_lower ON public.users (lower(user_email));
CREATE UNIQUE INDEX idx_users_name_lower ON public.users (lower(user_name));
CREATE INDEX idx_users_created_at ON public.users (user_created_at);
CREATE INDEX idx_users_updated_at ON public.users (user_updated_at);
CREATE INDEX idx_users_email_verified ON public.users (user_is_email_verified);
CREATE INDEX idx_users_country ON public.users (user_country);
CREATE INDEX idx_users_language ON public.users (user_language);
CREATE INDEX idx_users_subdivision ON public.users (user_subdivision);

CREATE TABLE public.roles (
    role_id UUID PRIMARY KEY DEFAULT uuidv7(),
    role_name TEXT NOT NULL UNIQUE,
    role_description TEXT
);

CREATE INDEX idx_roles_role_name ON public.roles (role_name);

CREATE TABLE public.permissions (
    permission_id UUID PRIMARY KEY DEFAULT uuidv7(),
    permission_name TEXT NOT NULL UNIQUE,
    permission_description TEXT
);

CREATE INDEX idx_permissions_permission_name ON public.permissions (permission_name);

CREATE TABLE public.role_permissions (
    role_permission_id UUID PRIMARY KEY DEFAULT uuidv7(),
    role_id UUID NOT NULL,
    permission_id UUID NOT NULL,
    CONSTRAINT fk_role_permissions_role FOREIGN KEY (role_id) REFERENCES public.roles (role_id) ON DELETE CASCADE,
    CONSTRAINT fk_role_permissions_permission FOREIGN KEY (permission_id) REFERENCES public.permissions (permission_id) ON DELETE CASCADE,
    UNIQUE (role_id, permission_id)
);

CREATE INDEX idx_role_permissions_role_id ON public.role_permissions (role_id);
CREATE INDEX idx_role_permissions_permission_id ON public.role_permissions (permission_id);

CREATE TABLE public.user_roles (
    user_role_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL UNIQUE,
    role_id UUID NOT NULL,
    CONSTRAINT fk_user_roles_user FOREIGN KEY (user_id) REFERENCES public.users (user_id) ON DELETE CASCADE,
    CONSTRAINT fk_user_roles_role FOREIGN KEY (role_id) REFERENCES public.roles (role_id) ON DELETE CASCADE
);

CREATE INDEX idx_user_roles_user_id ON public.user_roles (user_id);
CREATE INDEX idx_user_roles_role_id ON public.user_roles (role_id);

INSERT INTO public.roles (role_id, role_name, role_description)
VALUES
    ('019a6c86-8bca-7b91-b9c0-1d4cc96b3263', 'admin', 'Administrator role with total access; owner of the site.'),
    ('019a6c86-b163-7452-aa70-5997736b0434', 'moderator', 'Moderator role with elevated permissions'),
    ('019dd245-98e6-7b57-ade5-92019a275c6e', 'service_provider', 'Service provider client role for serving user clients'),
    ('019a6c86-bfa6-7903-9176-dc5f66f729fe', 'user', 'Client user role for requesting services'),
    ('019a6c86-d66b-7223-97ef-a8a26551a080', 'guest', 'Guest role with minimal access');

CREATE TABLE public.email_verification_tokens (
    email_verification_token_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL,
    email_verification_token UUID NOT NULL DEFAULT uuidv7(),
    email_verification_token_expires_at TIMESTAMPTZ NOT NULL,
    email_verification_token_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    email_verification_token_used_at TIMESTAMPTZ,
    CONSTRAINT fk_user_email_verification FOREIGN KEY (user_id) REFERENCES public.users (user_id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_email_verification_tokens_token ON public.email_verification_tokens (email_verification_token);
CREATE INDEX idx_email_verification_tokens_user_id ON public.email_verification_tokens (user_id);
CREATE INDEX idx_email_verification_tokens_created_at ON public.email_verification_tokens (email_verification_token_created_at);
CREATE INDEX idx_email_verification_tokens_expires_at ON public.email_verification_tokens (email_verification_token_expires_at);
CREATE INDEX idx_email_verification_tokens_used_at ON public.email_verification_tokens (email_verification_token_used_at);

CREATE TABLE public.password_reset_tokens (
    password_reset_token_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL,
    password_reset_token UUID NOT NULL DEFAULT uuidv7(),
    password_reset_token_expires_at TIMESTAMPTZ NOT NULL,
    password_reset_token_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    password_reset_token_used_at TIMESTAMPTZ,
    CONSTRAINT fk_user_password_reset FOREIGN KEY (user_id) REFERENCES public.users (user_id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_password_reset_tokens_token ON public.password_reset_tokens (password_reset_token);
CREATE INDEX idx_password_reset_tokens_user_id ON public.password_reset_tokens (user_id);
CREATE INDEX idx_password_reset_tokens_created_at ON public.password_reset_tokens (password_reset_token_created_at);
CREATE INDEX idx_password_reset_tokens_expires_at ON public.password_reset_tokens (password_reset_token_expires_at);
CREATE INDEX idx_password_reset_tokens_used_at ON public.password_reset_tokens (password_reset_token_used_at);
