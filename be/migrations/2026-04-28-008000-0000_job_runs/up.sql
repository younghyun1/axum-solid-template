CREATE TYPE public.job_run_status AS ENUM (
    'scheduled',
    'running',
    'succeeded',
    'failed',
    'cancelled',
    'skipped'
);

CREATE TABLE public.job_runs (
    job_run_id UUID PRIMARY KEY DEFAULT uuidv7(),
    job_run_name TEXT NOT NULL,
    job_run_status public.job_run_status NOT NULL,
    job_run_scheduled_for TIMESTAMPTZ NOT NULL,
    job_run_started_at TIMESTAMPTZ,
    job_run_finished_at TIMESTAMPTZ,
    job_run_duration_ms BIGINT,
    job_run_attempt INTEGER NOT NULL DEFAULT 1,
    job_run_error_code TEXT,
    job_run_error_message TEXT,
    job_run_metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    job_run_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    job_run_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT job_runs_attempt_positive CHECK (job_run_attempt > 0),
    CONSTRAINT job_runs_duration_non_negative CHECK (
        job_run_duration_ms IS NULL OR job_run_duration_ms >= 0
    ),
    CONSTRAINT job_runs_finished_after_started CHECK (
        job_run_started_at IS NULL
        OR job_run_finished_at IS NULL
        OR job_run_finished_at >= job_run_started_at
    ),
    CONSTRAINT job_runs_status_timestamps CHECK (
        (
            job_run_status = 'scheduled'
            AND job_run_started_at IS NULL
            AND job_run_finished_at IS NULL
        )
        OR (
            job_run_status = 'running'
            AND job_run_started_at IS NOT NULL
            AND job_run_finished_at IS NULL
        )
        OR (
            job_run_status IN ('succeeded', 'failed', 'cancelled')
            AND job_run_started_at IS NOT NULL
            AND job_run_finished_at IS NOT NULL
        )
        OR (
            job_run_status = 'skipped'
            AND job_run_finished_at IS NOT NULL
        )
    ),
    UNIQUE (job_run_name, job_run_scheduled_for, job_run_attempt)
);

CREATE INDEX idx_job_runs_name ON public.job_runs (job_run_name);
CREATE INDEX idx_job_runs_status_scheduled_for ON public.job_runs (job_run_status, job_run_scheduled_for);
CREATE INDEX idx_job_runs_scheduled_for ON public.job_runs (job_run_scheduled_for);
CREATE INDEX idx_job_runs_started_at ON public.job_runs (job_run_started_at);
CREATE INDEX idx_job_runs_finished_at ON public.job_runs (job_run_finished_at);
