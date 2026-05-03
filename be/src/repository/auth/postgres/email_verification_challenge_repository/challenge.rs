use chrono::{DateTime, Utc};
use diesel::sql_types::{Integer, Nullable, Text, Timestamptz, Uuid as SqlUuid};
use diesel::{OptionalExtension, sql_query};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::domain::auth::email_verification_challenge::{
    CHALLENGE_STATUS_EXPIRED, CHALLENGE_STATUS_FAILED, CHALLENGE_STATUS_ISSUED,
    CHALLENGE_STATUS_SOLVED, CHALLENGE_STATUS_SUPERSEDED, EmailVerificationChallengeRow,
    NewEmailVerificationChallenge,
};

pub async fn supersede_issued_challenges(
    conn: &mut AsyncPgConnection,
    token_id: Uuid,
) -> Result<usize, diesel::result::Error> {
    match sql_query(
        r#"
        UPDATE public.email_verification_challenges
        SET email_verification_challenge_status = $1
        WHERE
            email_verification_token_id = $2
            AND email_verification_challenge_status = $3
        "#,
    )
    .bind::<Text, _>(CHALLENGE_STATUS_SUPERSEDED)
    .bind::<SqlUuid, _>(token_id)
    .bind::<Text, _>(CHALLENGE_STATUS_ISSUED)
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}

pub async fn insert_challenge(
    conn: &mut AsyncPgConnection,
    challenge: NewEmailVerificationChallenge,
) -> Result<EmailVerificationChallengeRow, diesel::result::Error> {
    match sql_query(
        r#"
        INSERT INTO public.email_verification_challenges (
            email_verification_challenge_id,
            email_verification_token_id,
            email_verification_question_id,
            email_verification_challenge_pow_salt,
            email_verification_challenge_pow_difficulty_bits,
            email_verification_challenge_pow_algorithm,
            email_verification_challenge_minimum_elapsed_ms,
            email_verification_challenge_issued_at,
            email_verification_challenge_expires_at,
            email_verification_challenge_client_ip,
            email_verification_challenge_user_agent
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING
            email_verification_challenge_id,
            email_verification_token_id,
            email_verification_question_id,
            email_verification_challenge_pow_salt,
            email_verification_challenge_pow_difficulty_bits,
            email_verification_challenge_pow_algorithm,
            email_verification_challenge_minimum_elapsed_ms,
            email_verification_challenge_status,
            email_verification_challenge_issued_at,
            email_verification_challenge_expires_at,
            email_verification_challenge_solved_at,
            email_verification_challenge_failed_at,
            email_verification_challenge_attempt_count
        "#,
    )
    .bind::<SqlUuid, _>(challenge.email_verification_challenge_id)
    .bind::<SqlUuid, _>(challenge.email_verification_token_id)
    .bind::<SqlUuid, _>(challenge.email_verification_question_id)
    .bind::<Text, _>(challenge.email_verification_challenge_pow_salt)
    .bind::<Integer, _>(challenge.email_verification_challenge_pow_difficulty_bits)
    .bind::<Text, _>(challenge.email_verification_challenge_pow_algorithm)
    .bind::<Integer, _>(challenge.email_verification_challenge_minimum_elapsed_ms)
    .bind::<Timestamptz, _>(challenge.email_verification_challenge_issued_at)
    .bind::<Timestamptz, _>(challenge.email_verification_challenge_expires_at)
    .bind::<Nullable<Text>, _>(challenge.email_verification_challenge_client_ip)
    .bind::<Nullable<Text>, _>(challenge.email_verification_challenge_user_agent)
    .get_result::<EmailVerificationChallengeRow>(conn)
    .await
    {
        Ok(row) => Ok(row),
        Err(error) => Err(error),
    }
}

pub async fn find_challenge_by_id(
    conn: &mut AsyncPgConnection,
    challenge_id: Uuid,
) -> Result<Option<EmailVerificationChallengeRow>, diesel::result::Error> {
    match sql_query(
        r#"
        SELECT
            email_verification_challenge_id,
            email_verification_token_id,
            email_verification_question_id,
            email_verification_challenge_pow_salt,
            email_verification_challenge_pow_difficulty_bits,
            email_verification_challenge_pow_algorithm,
            email_verification_challenge_minimum_elapsed_ms,
            email_verification_challenge_status,
            email_verification_challenge_issued_at,
            email_verification_challenge_expires_at,
            email_verification_challenge_solved_at,
            email_verification_challenge_failed_at,
            email_verification_challenge_attempt_count
        FROM public.email_verification_challenges
        WHERE email_verification_challenge_id = $1
        FOR UPDATE
        "#,
    )
    .bind::<SqlUuid, _>(challenge_id)
    .get_result::<EmailVerificationChallengeRow>(conn)
    .await
    .optional()
    {
        Ok(challenge) => Ok(challenge),
        Err(error) => Err(error),
    }
}

pub async fn record_challenge_attempt(
    conn: &mut AsyncPgConnection,
    challenge_id: Uuid,
    last_error: &str,
) -> Result<usize, diesel::result::Error> {
    match sql_query(
        r#"
        UPDATE public.email_verification_challenges
        SET
            email_verification_challenge_attempt_count =
                email_verification_challenge_attempt_count + 1,
            email_verification_challenge_last_error = $1
        WHERE email_verification_challenge_id = $2
        "#,
    )
    .bind::<Text, _>(last_error)
    .bind::<SqlUuid, _>(challenge_id)
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}

pub async fn mark_challenge_solved(
    conn: &mut AsyncPgConnection,
    challenge_id: Uuid,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    update_challenge_terminal_status(conn, challenge_id, CHALLENGE_STATUS_SOLVED, now).await
}

pub async fn mark_challenge_failed(
    conn: &mut AsyncPgConnection,
    challenge_id: Uuid,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    update_challenge_terminal_status(conn, challenge_id, CHALLENGE_STATUS_FAILED, now).await
}

pub async fn mark_challenge_expired(
    conn: &mut AsyncPgConnection,
    challenge_id: Uuid,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    update_challenge_terminal_status(conn, challenge_id, CHALLENGE_STATUS_EXPIRED, now).await
}

async fn update_challenge_terminal_status(
    conn: &mut AsyncPgConnection,
    challenge_id: Uuid,
    status: &str,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    let solved_at = match status {
        CHALLENGE_STATUS_SOLVED => Some(now),
        _ => None,
    };
    let failed_at = match status {
        CHALLENGE_STATUS_FAILED | CHALLENGE_STATUS_EXPIRED => Some(now),
        _ => None,
    };

    match sql_query(
        r#"
        UPDATE public.email_verification_challenges
        SET
            email_verification_challenge_status = $1,
            email_verification_challenge_solved_at = $2,
            email_verification_challenge_failed_at = $3
        WHERE email_verification_challenge_id = $4
        "#,
    )
    .bind::<Text, _>(status)
    .bind::<Nullable<Timestamptz>, _>(solved_at)
    .bind::<Nullable<Timestamptz>, _>(failed_at)
    .bind::<SqlUuid, _>(challenge_id)
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}
