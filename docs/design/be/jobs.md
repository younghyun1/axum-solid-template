# Job Scheduling

The backend should use a small in-process Tokio scheduler, not an external worker service. The deployment target is a single-server on-prem install, so PostgreSQL is the durable coordination point and no Redis or Valkey dependency is required.

## Runtime Model

Job definitions are code-first. A job registers a stable name, an interval, and an async handler.

The scheduler API should be shaped around a small interval enum:

```rust
pub enum JobInterval {
    Every(Duration),
    Hourly { minute: u8, second: u8 },
    Daily { time: chrono::NaiveTime, timezone: chrono_tz::Tz },
    Monthly { day: u8, time: chrono::NaiveTime, timezone: chrono_tz::Tz },
}
```

The runner should expose a registration surface equivalent to:

```rust
run(job_name, first_run_at, interval, handler)
```

`Every(Duration)` is monotonic and should use `tokio::time::interval`. Wall-clock intervals must compute the next calendar time and then use `tokio::time::sleep_until`; fixed seconds are not acceptable for daily or monthly jobs because DST and month lengths are calendar concerns.

Jobs must be idempotent. Startup may see a previously `running` job if the server exited mid-run, and the recovery path should mark it `failed` or schedule a retry before launching new work.

## Logging

Each job run should write structured tracing logs:

- `job_name`
- `job_run_id`
- `job_run_status`
- `scheduled_for`
- `attempt`
- `elapsed = ?duration`
- `error = %error` on failure

Use `?Duration` for human-readable elapsed timing in logs. Avoid integer-only elapsed fields unless the value is intended for metrics aggregation.

## Database Schema

The durable log table is `job_runs`. It records each scheduled execution attempt, not just failures.

Columns:

- `job_run_id UUID PRIMARY KEY DEFAULT uuidv7()`
- `job_run_name TEXT NOT NULL`
- `job_run_status public.job_run_status NOT NULL`
- `job_run_scheduled_for TIMESTAMPTZ NOT NULL`
- `job_run_started_at TIMESTAMPTZ`
- `job_run_finished_at TIMESTAMPTZ`
- `job_run_duration_ms BIGINT`
- `job_run_attempt INTEGER NOT NULL DEFAULT 1`
- `job_run_error_code TEXT`
- `job_run_error_message TEXT`
- `job_run_metadata JSONB NOT NULL DEFAULT '{}'::jsonb`
- `job_run_created_at TIMESTAMPTZ NOT NULL DEFAULT now()`
- `job_run_updated_at TIMESTAMPTZ NOT NULL DEFAULT now()`

`job_run_status` is a PostgreSQL enum:

- `scheduled`
- `running`
- `succeeded`
- `failed`
- `cancelled`
- `skipped`

Constraints:

- `(job_run_name, job_run_scheduled_for, job_run_attempt)` is unique.
- `job_run_attempt > 0`.
- `job_run_duration_ms` is null or non-negative.
- finished timestamps must not precede started timestamps.
- status-specific timestamp rules are enforced in the migration.

Indexes:

- `job_run_name`
- `(job_run_status, job_run_scheduled_for)`
- `job_run_scheduled_for`
- `job_run_started_at`
- `job_run_finished_at`

The initial migration is `2026-04-28-008000-0000_job_runs`.
