use chrono::{naive::NaiveDateTime, offset::Utc};
use serde::{Deserialize, Serialize};
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Failure {
    pub failure_id: String,
    // pgsql default is 25
    pub max_attempts: i32,
    pub attempt_count: i32,
    pub last_attempt: NaiveDateTime,
    pub next_attempt: NaiveDateTime,
}

impl Failure {
    pub fn new(failure_id: impl Into<String>) -> Self {
        let now = Utc::now().naive_utc();

        Self {
            failure_id: failure_id.into(),
            max_attempts: 25,
            attempt_count: 1,
            last_attempt: now,
            // Not used for inserts, just a dummy value for now
            next_attempt: now,
        }
    }

    pub fn max_attempts(mut self, max_attempts: i32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    pub fn inc(&mut self) {
        self.attempt_count += 1;
    }

    /// If this has been failed max_attempts times. If this is reached, it
    /// should not be attempted again
    pub fn exceeded_attempts(&self) -> bool {
        self.attempt_count >= self.max_attempts
    }

    /// If next_attempt timestamp is past, enough time waited to attempt again
    pub fn should_attempt(&self) -> bool {
        Utc::now().naive_utc() > self.next_attempt
    }

    pub async fn from_id(ctx: &Context, failure_id: &str) -> Result<Option<Self>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        get_from_id_query(&pool, failure_id).await
    }

    pub async fn save(&self, ctx: &Context) -> Result<Self> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        upsert_query(&pool, &self).await
    }

    pub async fn delete(&self, ctx: &Context) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        delete_failure_query(&pool, &self.failure_id).await
    }

    pub async fn delete_id(ctx: &Context, failure_id: &str) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        delete_failure_query(&pool, failure_id).await
    }
}

async fn get_from_id_query(pool: &sqlx::PgPool, failure_id: &str) -> Result<Option<Failure>> {
    sqlx::query_as!(
        Failure,
        r#"
            SELECT *
              FROM app_hidden.failures
             WHERE failure_id = $1
        "#,
        failure_id
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn upsert_query(pool: &sqlx::PgPool, failure: &Failure) -> Result<Failure> {
    sqlx::query_as!(
        Failure,
        r#"
        INSERT INTO app_hidden.failures (
                        failure_id, max_attempts, attempt_count, last_attempt
                    )
             VALUES ($1, $2, $3, NOW())
        ON CONFLICT (failure_id)
          DO UPDATE
                SET attempt_count = $3,
                    last_attempt = NOW()
            RETURNING *
        "#,
        failure.failure_id,
        failure.max_attempts,
        failure.attempt_count,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

async fn delete_failure_query(pool: &sqlx::PgPool, failure_id: &str) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM app_hidden.failures
                  WHERE failure_id = $1
        "#,
        failure_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
