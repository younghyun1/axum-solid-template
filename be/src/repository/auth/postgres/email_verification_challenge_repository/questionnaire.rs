use diesel::{OptionalExtension, sql_query};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::domain::auth::email_verification_challenge::{
    EmailVerificationQuestion, EmailVerificationQuestionAnswer, EmailVerificationQuestionAnswerRow,
    EmailVerificationQuestionnaireRevisionRow, EmailVerificationQuestionnaireSnapshot,
};

pub async fn load_questionnaire_snapshot(
    conn: &mut AsyncPgConnection,
) -> Result<EmailVerificationQuestionnaireSnapshot, diesel::result::Error> {
    let revision = match sql_query(
        r#"
        SELECT email_verification_questionnaire_revision
        FROM public.email_verification_questionnaire_state
        ORDER BY email_verification_questionnaire_updated_at DESC
        LIMIT 1
        "#,
    )
    .get_result::<EmailVerificationQuestionnaireRevisionRow>(conn)
    .await
    .optional()
    {
        Ok(Some(row)) => row.email_verification_questionnaire_revision,
        Ok(None) => 0,
        Err(error) => return Err(error),
    };

    let rows = match sql_query(
        r#"
        SELECT
            q.email_verification_question_id,
            q.email_verification_question_prompt,
            a.email_verification_question_answer_id,
            a.email_verification_question_answer_text,
            a.email_verification_question_answer_normalized
        FROM public.email_verification_questions q
        LEFT JOIN public.email_verification_question_answers a
            ON a.email_verification_question_id = q.email_verification_question_id
            AND a.email_verification_question_answer_status = 'active'
        WHERE q.email_verification_question_status = 'active'
        ORDER BY
            q.email_verification_question_created_at ASC,
            a.email_verification_question_answer_created_at ASC
        "#,
    )
    .get_results::<EmailVerificationQuestionAnswerRow>(conn)
    .await
    {
        Ok(rows) => rows,
        Err(error) => return Err(error),
    };

    let mut questions: Vec<EmailVerificationQuestion> = Vec::new();
    for row in rows {
        let question_index = match questions.iter().position(|question| {
            question.email_verification_question_id == row.email_verification_question_id
        }) {
            Some(index) => index,
            None => {
                questions.push(EmailVerificationQuestion {
                    email_verification_question_id: row.email_verification_question_id,
                    email_verification_question_prompt: row.email_verification_question_prompt,
                    email_verification_question_answers: Vec::new(),
                });
                questions.len() - 1
            }
        };

        let answer_id = match row.email_verification_question_answer_id {
            Some(answer_id) => answer_id,
            None => continue,
        };
        let answer_text = match row.email_verification_question_answer_text {
            Some(answer_text) => answer_text,
            None => continue,
        };
        let answer_normalized = match row.email_verification_question_answer_normalized {
            Some(answer_normalized) => answer_normalized,
            None => continue,
        };
        let question = match questions.get_mut(question_index) {
            Some(question) => question,
            None => continue,
        };
        question
            .email_verification_question_answers
            .push(EmailVerificationQuestionAnswer {
                email_verification_question_answer_id: answer_id,
                email_verification_question_answer_text: answer_text,
                email_verification_question_answer_normalized: answer_normalized,
            });
    }

    Ok(EmailVerificationQuestionnaireSnapshot {
        email_verification_questionnaire_revision: revision,
        email_verification_questions: questions,
    })
}

pub async fn bump_questionnaire_revision(
    conn: &mut AsyncPgConnection,
) -> Result<i64, diesel::result::Error> {
    match sql_query(
        r#"
        UPDATE public.email_verification_questionnaire_state
        SET
            email_verification_questionnaire_revision =
                email_verification_questionnaire_revision + 1,
            email_verification_questionnaire_updated_at = now()
        RETURNING email_verification_questionnaire_revision
        "#,
    )
    .get_result::<EmailVerificationQuestionnaireRevisionRow>(conn)
    .await
    {
        Ok(row) => Ok(row.email_verification_questionnaire_revision),
        Err(error) => Err(error),
    }
}
