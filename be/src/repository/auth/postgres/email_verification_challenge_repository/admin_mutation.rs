use chrono::{DateTime, Utc};
use diesel::sql_query;
use diesel::sql_types::{Text, Timestamptz, Uuid as SqlUuid};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::domain::auth::email_verification_challenge::{
    ANSWER_STATUS_ACTIVE, ANSWER_STATUS_DELETED, CreatedEmailVerificationQuestionAnswerRow,
    CreatedEmailVerificationQuestionRow, QUESTION_STATUS_ACTIVE, QUESTION_STATUS_DELETED,
};

pub async fn insert_question(
    conn: &mut AsyncPgConnection,
    prompt: &str,
    admin_user_id: Uuid,
) -> Result<Uuid, diesel::result::Error> {
    match sql_query(
        r#"
        INSERT INTO public.email_verification_questions (
            email_verification_question_prompt,
            email_verification_question_created_by
        )
        VALUES ($1, $2)
        RETURNING email_verification_question_id
        "#,
    )
    .bind::<Text, _>(prompt)
    .bind::<SqlUuid, _>(admin_user_id)
    .get_result::<CreatedEmailVerificationQuestionRow>(conn)
    .await
    {
        Ok(row) => Ok(row.email_verification_question_id),
        Err(error) => Err(error),
    }
}

pub async fn insert_answer(
    conn: &mut AsyncPgConnection,
    question_id: Uuid,
    answer_text: &str,
    answer_normalized: &str,
    admin_user_id: Uuid,
) -> Result<Uuid, diesel::result::Error> {
    match sql_query(
        r#"
        INSERT INTO public.email_verification_question_answers (
            email_verification_question_id,
            email_verification_question_answer_text,
            email_verification_question_answer_normalized,
            email_verification_question_answer_created_by
        )
        VALUES ($1, $2, $3, $4)
        RETURNING email_verification_question_answer_id
        "#,
    )
    .bind::<SqlUuid, _>(question_id)
    .bind::<Text, _>(answer_text)
    .bind::<Text, _>(answer_normalized)
    .bind::<SqlUuid, _>(admin_user_id)
    .get_result::<CreatedEmailVerificationQuestionAnswerRow>(conn)
    .await
    {
        Ok(row) => Ok(row.email_verification_question_answer_id),
        Err(error) => Err(error),
    }
}

pub async fn delete_question(
    conn: &mut AsyncPgConnection,
    question_id: Uuid,
    admin_user_id: Uuid,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    match sql_query(
        r#"
        UPDATE public.email_verification_questions
        SET
            email_verification_question_status = $1,
            email_verification_question_deleted_at = $2,
            email_verification_question_deleted_by = $3,
            email_verification_question_updated_at = $2
        WHERE
            email_verification_question_id = $4
            AND email_verification_question_status = $5
        "#,
    )
    .bind::<Text, _>(QUESTION_STATUS_DELETED)
    .bind::<Timestamptz, _>(now)
    .bind::<SqlUuid, _>(admin_user_id)
    .bind::<SqlUuid, _>(question_id)
    .bind::<Text, _>(QUESTION_STATUS_ACTIVE)
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}

pub async fn delete_question_answers(
    conn: &mut AsyncPgConnection,
    question_id: Uuid,
    admin_user_id: Uuid,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    match sql_query(
        r#"
        UPDATE public.email_verification_question_answers
        SET
            email_verification_question_answer_status = $1,
            email_verification_question_answer_deleted_at = $2,
            email_verification_question_answer_deleted_by = $3,
            email_verification_question_answer_updated_at = $2
        WHERE
            email_verification_question_id = $4
            AND email_verification_question_answer_status = $5
        "#,
    )
    .bind::<Text, _>(ANSWER_STATUS_DELETED)
    .bind::<Timestamptz, _>(now)
    .bind::<SqlUuid, _>(admin_user_id)
    .bind::<SqlUuid, _>(question_id)
    .bind::<Text, _>(ANSWER_STATUS_ACTIVE)
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}

pub async fn delete_answer(
    conn: &mut AsyncPgConnection,
    question_id: Uuid,
    answer_id: Uuid,
    admin_user_id: Uuid,
    now: DateTime<Utc>,
) -> Result<usize, diesel::result::Error> {
    match sql_query(
        r#"
        UPDATE public.email_verification_question_answers
        SET
            email_verification_question_answer_status = $1,
            email_verification_question_answer_deleted_at = $2,
            email_verification_question_answer_deleted_by = $3,
            email_verification_question_answer_updated_at = $2
        WHERE
            email_verification_question_id = $4
            AND email_verification_question_answer_id = $5
            AND email_verification_question_answer_status = $6
        "#,
    )
    .bind::<Text, _>(ANSWER_STATUS_DELETED)
    .bind::<Timestamptz, _>(now)
    .bind::<SqlUuid, _>(admin_user_id)
    .bind::<SqlUuid, _>(question_id)
    .bind::<SqlUuid, _>(answer_id)
    .bind::<Text, _>(ANSWER_STATUS_ACTIVE)
    .execute(conn)
    .await
    {
        Ok(rows) => Ok(rows),
        Err(error) => Err(error),
    }
}
