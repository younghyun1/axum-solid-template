CREATE TABLE public.auth_refresh_sessions (
    auth_refresh_session_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL,
    auth_refresh_session_token_hash VARCHAR(64) NOT NULL,
    auth_refresh_session_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    auth_refresh_session_expires_at TIMESTAMPTZ NOT NULL,
    auth_refresh_session_last_used_at TIMESTAMPTZ,
    auth_refresh_session_rotated_at TIMESTAMPTZ,
    auth_refresh_session_revoked_at TIMESTAMPTZ,
    auth_refresh_session_user_auth_token_version INTEGER NOT NULL,
    CONSTRAINT fk_auth_refresh_sessions_user
        FOREIGN KEY (user_id) REFERENCES public.users(user_id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_auth_refresh_sessions_token_hash
    ON public.auth_refresh_sessions (auth_refresh_session_token_hash);
CREATE INDEX idx_auth_refresh_sessions_user_id
    ON public.auth_refresh_sessions (user_id);
CREATE INDEX idx_auth_refresh_sessions_expires_at
    ON public.auth_refresh_sessions (auth_refresh_session_expires_at);
CREATE INDEX idx_auth_refresh_sessions_revoked_at
    ON public.auth_refresh_sessions (auth_refresh_session_revoked_at);
