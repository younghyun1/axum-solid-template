use std::{
    collections::HashMap,
    fmt,
    time::{Duration, Instant},
};

use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::{
    domain::auth::email_verification_challenge::{
        EmailVerificationChallengeRow, EmailVerificationQuestion,
        EmailVerificationQuestionnaireSnapshot,
    },
    init::db_pool::{DbPool, DbPoolInitError, get_conn},
    repository::auth::postgres::email_verification_challenge_repository::questionnaire,
};

#[derive(Debug)]
pub struct EmailVerificationChallengeCache {
    questionnaire: RwLock<EmailVerificationQuestionnaireSnapshot>,
    active_challenges: RwLock<HashMap<Uuid, EmailVerificationChallengeRow>>,
    rate_limits: RwLock<HashMap<String, RateLimitBucket>>,
}

#[derive(Debug)]
pub enum EmailVerificationChallengeCacheError {
    DbPool { error: DbPoolInitError },
    Query { error: String },
}

#[derive(Debug, Clone)]
struct RateLimitBucket {
    window_started_at: Instant,
    count: u32,
}

impl EmailVerificationChallengeCache {
    /// Perform the `load` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `db_pool` -
    /// # Returns
    /// A `Result`, either containing the function output or an error.
    pub async fn load(db_pool: &DbPool) -> Result<Self, EmailVerificationChallengeCacheError> {
        let mut conn = match get_conn(db_pool).await {
            Ok(conn) => conn,
            Err(error) => return Err(EmailVerificationChallengeCacheError::DbPool { error }),
        };
        let questionnaire = match questionnaire::load_questionnaire_snapshot(&mut conn).await {
            Ok(questionnaire) => questionnaire,
            Err(error) => {
                return Err(EmailVerificationChallengeCacheError::Query {
                    error: error.to_string(),
                });
            }
        };

        info!(
            questionnaire_revision = questionnaire.email_verification_questionnaire_revision,
            questions = questionnaire.email_verification_questions.len(),
            "Loaded email verification questionnaire cache"
        );

        Ok(Self {
            questionnaire: RwLock::new(questionnaire),
            active_challenges: RwLock::new(HashMap::new()),
            rate_limits: RwLock::new(HashMap::new()),
        })
    }

    /// Perform the `questionnaire_snapshot` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub async fn questionnaire_snapshot(&self) -> EmailVerificationQuestionnaireSnapshot {
        self.questionnaire.read().await.clone()
    }

    pub async fn replace_questionnaire(
        &self,
        questionnaire: EmailVerificationQuestionnaireSnapshot,
    ) {
        {
            let mut current = self.questionnaire.write().await;
            *current = questionnaire.clone();
        }

        info!(
            questionnaire_revision = questionnaire.email_verification_questionnaire_revision,
            questions = questionnaire.email_verification_questions.len(),
            "Updated email verification questionnaire cache"
        );
    }

    pub async fn refresh_questionnaire(
        &self,
        db_pool: &DbPool,
    ) -> Result<EmailVerificationQuestionnaireSnapshot, EmailVerificationChallengeCacheError> {
        let mut conn = match get_conn(db_pool).await {
            Ok(conn) => conn,
            Err(error) => return Err(EmailVerificationChallengeCacheError::DbPool { error }),
        };
        let questionnaire = match questionnaire::load_questionnaire_snapshot(&mut conn).await {
            Ok(questionnaire) => questionnaire,
            Err(error) => {
                return Err(EmailVerificationChallengeCacheError::Query {
                    error: error.to_string(),
                });
            }
        };

        {
            let mut current = self.questionnaire.write().await;
            *current = questionnaire.clone();
        }

        info!(
            questionnaire_revision = questionnaire.email_verification_questionnaire_revision,
            questions = questionnaire.email_verification_questions.len(),
            "Refreshed email verification questionnaire cache"
        );

        Ok(questionnaire)
    }

    /// Perform the `store_challenge` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `challenge` -
    /// # Returns
    /// No value is returned (`()`).
    pub async fn store_challenge(&self, challenge: EmailVerificationChallengeRow) {
        self.active_challenges
            .write()
            .await
            .insert(challenge.email_verification_challenge_id, challenge);
    }

    /// Perform the `challenge` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `challenge_id` -
    /// # Returns
    /// Returns the value produced by this function.
    pub async fn challenge(&self, challenge_id: Uuid) -> Option<EmailVerificationChallengeRow> {
        self.active_challenges
            .read()
            .await
            .get(&challenge_id)
            .cloned()
    }

    /// Perform the `remove_challenge` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `challenge_id` -
    /// # Returns
    /// No value is returned (`()`).
    pub async fn remove_challenge(&self, challenge_id: Uuid) {
        self.active_challenges.write().await.remove(&challenge_id);
    }

    pub async fn clear_runtime_state(&self) {
        self.active_challenges.write().await.clear();
        self.rate_limits.write().await.clear();
    }

    /// Perform the `check_rate_limit` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `key` -
    /// * `limit` -
    /// * `window` -
    /// # Returns
    /// Returns the value produced by this function.
    pub async fn check_rate_limit(&self, key: String, limit: u32, window: Duration) -> bool {
        let now = Instant::now();
        let mut rate_limits = self.rate_limits.write().await;
        let bucket = match rate_limits.get_mut(&key) {
            Some(bucket) => bucket,
            None => {
                rate_limits.insert(
                    key,
                    RateLimitBucket {
                        window_started_at: now,
                        count: 1,
                    },
                );
                return true;
            }
        };

        if bucket.window_started_at.elapsed() > window {
            bucket.window_started_at = now;
            bucket.count = 1;
            return true;
        }

        if bucket.count >= limit {
            return false;
        }

        bucket.count += 1;
        true
    }
}

impl EmailVerificationQuestionnaireSnapshot {
    /// Perform the `public_questions` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn public_questions(&self) -> Vec<EmailVerificationQuestion> {
        self.email_verification_questions
            .iter()
            .filter(|question| !question.email_verification_question_answers.is_empty())
            .cloned()
            .collect()
    }
}

impl fmt::Display for EmailVerificationChallengeCacheError {
    /// Perform the `fmt` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `self` -
    /// * `formatter` -
    /// # Returns
    /// Returns the value produced by this function.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbPool { error } => {
                write!(
                    formatter,
                    "failed to get DB connection for challenge cache: {error}"
                )
            }
            Self::Query { error } => {
                write!(formatter, "failed to query challenge cache data: {error}")
            }
        }
    }
}
