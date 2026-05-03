import { createEffect, createSignal, For, Show } from "solid-js";

import {
  createEmailVerificationQuestion,
  createEmailVerificationQuestionAnswer,
  deleteEmailVerificationQuestion,
  deleteEmailVerificationQuestionAnswer,
  getEmailVerificationQuestions
} from "../../api/appApi";
import type { ApiCallResult, EmailVerificationQuestionnaireResponse } from "../../api/types";
import { NoticeView, SpinnerStatus } from "../shared/Feedback";
import { emptyNotice, type Notice } from "../shared/types";

interface AdminVerificationQuestionsPageProps {
  readonly isAdmin: boolean;
  readonly token: string;
  readonly onHome: () => void;
  readonly onSignIn: () => void;
}

export function AdminVerificationQuestionsPage(props: AdminVerificationQuestionsPageProps) {
  const [questionnaire, setQuestionnaire] =
    createSignal<EmailVerificationQuestionnaireResponse | null>(null);
  const [questionPrompt, setQuestionPrompt] = createSignal("");
  const [questionAnswers, setQuestionAnswers] = createSignal("");
  const [answerInputs, setAnswerInputs] = createSignal<Record<string, string>>({});
  const [notice, setNotice] = createSignal<Notice>(emptyNotice);
  const [running, setRunning] = createSignal(false);
  let loaded = false;

  createEffect(() => {
    if (loaded || !props.isAdmin || props.token.trim().length === 0) {
      return;
    }
    loaded = true;
    void loadQuestionnaire();
  });

  const loadQuestionnaire = async () => {
    setRunning(true);
    const result = await getEmailVerificationQuestions(props.token);
    setRunning(false);
    if (!result.ok || result.data === null) {
      setNotice({
        kind: "error",
        text: result.ok ? "Questionnaire response was empty." : result.error.message
      });
      return;
    }
    setQuestionnaire(result.data);
  };

  const replaceQuestionnaire = (result: ApiCallResult<EmailVerificationQuestionnaireResponse>) => {
    setRunning(false);
    if (!result.ok || result.data === null) {
      setNotice({
        kind: "error",
        text: result.ok ? "Questionnaire response was empty." : result.error.message
      });
      return;
    }
    setQuestionnaire(result.data);
    setNotice({ kind: "success", text: "Questionnaire updated." });
  };

  const createQuestion = async (event: SubmitEvent) => {
    event.preventDefault();
    const answers = questionAnswers()
      .split("\n")
      .map((answerText) => answerText.trim())
      .filter((answerText) => answerText.length > 0);
    setRunning(true);
    const result = await createEmailVerificationQuestion(props.token, {
      email_verification_question_answers: answers,
      email_verification_question_prompt: questionPrompt().trim()
    });
    if (result.ok) {
      setQuestionPrompt("");
      setQuestionAnswers("");
    }
    replaceQuestionnaire(result);
  };

  const addAnswer = async (questionId: string) => {
    const answerText = (answerInputs()[questionId] ?? "").trim();
    if (answerText.length === 0) {
      setNotice({ kind: "error", text: "Answer must not be empty." });
      return;
    }
    setRunning(true);
    const result = await createEmailVerificationQuestionAnswer(props.token, questionId, {
      email_verification_question_answer_text: answerText
    });
    if (result.ok) {
      setAnswerInputs((current) => ({ ...current, [questionId]: "" }));
    }
    replaceQuestionnaire(result);
  };

  const removeQuestion = async (questionId: string) => {
    setRunning(true);
    replaceQuestionnaire(await deleteEmailVerificationQuestion(props.token, questionId));
  };

  const removeAnswer = async (questionId: string, answerId: string) => {
    setRunning(true);
    replaceQuestionnaire(
      await deleteEmailVerificationQuestionAnswer(props.token, questionId, answerId)
    );
  };

  return (
    <section class="page-view admin-layout">
      <Show
        when={props.isAdmin && props.token.trim().length > 0}
        fallback={
          <div class="auth-card auth-card--narrow">
            <p class="eyebrow">Admin</p>
            <h1>Admin access required</h1>
            <p class="hero-text">Sign in as an admin to manage verification challenges.</p>
            <button class="primary-button" type="button" onClick={props.onSignIn}>
              Sign in
            </button>
            <button class="secondary-button" type="button" onClick={props.onHome}>
              Home
            </button>
          </div>
        }
      >
        <>
          <div class="section-heading">
            <p class="eyebrow">Admin</p>
            <h1>Verification challenges</h1>
          </div>
          <form class="auth-card admin-form" onSubmit={createQuestion}>
            <h2>New question</h2>
            <input
              aria-label="Question prompt"
              placeholder="Question prompt"
              required
              value={questionPrompt()}
              onInput={(event) => setQuestionPrompt(event.currentTarget.value)}
            />
            <textarea
              aria-label="Accepted answers"
              placeholder="Accepted answers, one per line"
              required
              value={questionAnswers()}
              onInput={(event) => setQuestionAnswers(event.currentTarget.value)}
            />
            <button class="primary-button" disabled={running()} type="submit">
              Create question
            </button>
          </form>
          <div class="admin-toolbar">
            <button
              class="secondary-button"
              disabled={running()}
              type="button"
              onClick={loadQuestionnaire}
            >
              Refresh
            </button>
            <span>
              Revision {questionnaire()?.email_verification_questionnaire_revision.toString() ?? "-"}
            </span>
          </div>
          <NoticeView notice={notice()} />
          <Show when={running()}>
            <SpinnerStatus text="Updating questionnaire" />
          </Show>
          <div class="question-list">
            <For each={questionnaire()?.email_verification_questions ?? []}>
              {(question) => (
                <article class="question-item">
                  <div class="question-item__header">
                    <h2>{question.email_verification_question_prompt}</h2>
                    <button
                      class="secondary-button"
                      disabled={running()}
                      type="button"
                      onClick={() => void removeQuestion(question.email_verification_question_id)}
                    >
                      Delete
                    </button>
                  </div>
                  <div class="answer-list">
                    <For each={question.email_verification_question_answers}>
                      {(answerItem) => (
                        <div class="answer-item">
                          <span>{answerItem.email_verification_question_answer_text}</span>
                          <button
                            class="secondary-button"
                            disabled={running()}
                            type="button"
                            onClick={() =>
                              void removeAnswer(
                                question.email_verification_question_id,
                                answerItem.email_verification_question_answer_id
                              )
                            }
                          >
                            Delete
                          </button>
                        </div>
                      )}
                    </For>
                  </div>
                  <div class="answer-add-row">
                    <input
                      aria-label="New accepted answer"
                      placeholder="New accepted answer"
                      value={answerInputs()[question.email_verification_question_id] ?? ""}
                      onInput={(event) =>
                        setAnswerInputs((current) => ({
                          ...current,
                          [question.email_verification_question_id]: event.currentTarget.value
                        }))
                      }
                    />
                    <button
                      class="secondary-button"
                      disabled={running()}
                      type="button"
                      onClick={() => void addAnswer(question.email_verification_question_id)}
                    >
                      Add answer
                    </button>
                  </div>
                </article>
              )}
            </For>
          </div>
        </>
      </Show>
    </section>
  );
}
