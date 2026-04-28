# Job System Plan

This plan covers the next job-system changes after the initial `job_runs` durable history table.

## Current State

- `be/src/job/mod.rs` exists but has no scheduler implementation yet.
- The current migration creates only `job_runs`.
- `job_runs.job_run_name` is currently the durable job discriminator.
- `job_runs` records execution attempts through `job_run_status`, scheduled/start/finish timestamps,
  duration, attempt number, optional error details, and JSON metadata.
- `docs/design/be/jobs.md` describes a code-first in-process Tokio scheduler, but the schema does
  not yet have a durable job definition table.

## Problem

`job_run_name` should not be the durable relationship key for job history. A text discriminator is
fragile for renames, joins, and future job-level metadata. The name should be a stable unique
attribute of a `jobs` table, while `job_runs` should reference jobs by UUID.

Target relationship:

- `jobs.job_id UUID PRIMARY KEY DEFAULT uuidv7()`
- `jobs.job_name TEXT NOT NULL UNIQUE`
- `job_runs.job_id UUID NOT NULL REFERENCES jobs(job_id)`

The runtime can still register jobs by stable code names, and logs should still include
`job_name`, but persistence should join through `job_id`.

## Target Schema

Add a `jobs` table:

- `job_id UUID PRIMARY KEY DEFAULT uuidv7()`
- `job_name TEXT NOT NULL UNIQUE`
- `job_description TEXT`
- `job_enabled BOOLEAN NOT NULL DEFAULT true`
- `job_created_at TIMESTAMPTZ NOT NULL DEFAULT now()`
- `job_updated_at TIMESTAMPTZ NOT NULL DEFAULT now()`

Change `job_runs`:

- Add `job_id UUID NOT NULL REFERENCES jobs(job_id)`.
- Replace the unique constraint on `(job_run_name, job_run_scheduled_for, job_run_attempt)` with
  `(job_id, job_run_scheduled_for, job_run_attempt)`.
- Replace `idx_job_runs_name` with an index on `job_id`.
- Remove `job_run_name` after the application code reads job names through the `jobs` join.

Keep `job_run_metadata` on `job_runs`; job-specific static metadata belongs in `jobs` only when it
is needed for queries, admin display, or scheduler behavior.

## Migration Approach

Use a staged migration so existing run history remains valid:

1. Create `jobs`.
2. Backfill `jobs` from distinct `job_runs.job_run_name`.
3. Add nullable `job_runs.job_id`.
4. Populate `job_runs.job_id` by joining `job_runs.job_run_name` to `jobs.job_name`.
5. Set `job_runs.job_id` to `NOT NULL`.
6. Add the `job_runs.job_id` foreign key.
7. Replace the unique constraint and indexes.
8. Update Diesel schema and backend code to use `job_id`.
9. Drop `job_runs.job_run_name` in a follow-up migration once code no longer depends on it.

The first implementation can combine these steps if no production data exists yet, but the
migration should still be written in a way that preserves local development data.

## Runtime Behavior

- Job registration remains code-first: each job registers a stable `job_name`, interval, and handler.
- Scheduler startup upserts one row per registered job in `jobs`.
- The scheduler keeps an in-memory `job_name -> job_id` map after startup.
- Job run creation writes `job_runs.job_id`, not `job_run_name`.
- Structured tracing logs should include `job_id`, `job_name`, and `job_run_id`.
- Recovery logic for abandoned `running` jobs should join through `job_id` and may include
  `jobs.job_name` in logs.

## Backend Work Items

1. Add the migration for `jobs` and `job_runs.job_id`.
2. Regenerate or update `be/src/schema.rs`.
3. Add domain newtypes for `JobId`, `JobName`, and `JobRunId` under `be/src/domain`.
4. Add DTOs for any exposed admin or diagnostic job endpoints under `be/src/dto`.
5. Add PostgreSQL repository functions for:
   - upserting registered jobs by name
   - creating scheduled job runs
   - transitioning run status
   - recovering abandoned running jobs
   - reading job run history joined to `jobs`
6. Implement the in-process Tokio scheduler described in `docs/design/be/jobs.md`.
7. Add unit tests for interval calculation and repository-level integration tests where practical.
8. Document any admin or diagnostic endpoints with `utoipa`.

## Open Decisions

- Whether `jobs` should store schedule configuration or remain only a job identity/catalog table.
- Whether disabled jobs should be skipped by runtime registration or persisted as `skipped` runs.
- Whether job renames should create a new `jobs` row or support an explicit alias/rename migration.
- Whether retry policy belongs in code only or should be queryable from `jobs`.
