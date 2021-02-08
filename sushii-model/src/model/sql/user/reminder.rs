use chrono::{naive::NaiveDateTime, offset::Utc, Duration};
use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::result::Result as StdResult;

use crate::error::Result;
use crate::keys::DbPool;
use crate::utils::duration::{find_duration, parse_duration};

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Reminder {
    pub user_id: i64,
    pub description: String,
    pub set_at: NaiveDateTime,
    pub expire_at: NaiveDateTime,
}

impl Reminder {
    pub fn new(user_id: UserId, desc_and_dur: &str) -> StdResult<Self, String> {
        // Look for position of duration string
        let duration_match = find_duration(desc_and_dur).ok_or("No duration given")?;

        // Description without the duration string
        let description = desc_and_dur
            .replace(duration_match.as_str(), "")
            .trim()
            .to_string();

        // Parsed duration
        let duration = parse_duration(duration_match.as_str())?;

        let set_at = Utc::now().naive_utc();
        let expire_at = set_at + duration;

        Ok(Self {
            user_id: user_id.into(),
            description,
            set_at,
            expire_at,
        })
    }

    pub fn get_duration(&self) -> Duration {
        Duration::seconds(
            self.expire_at
                .signed_duration_since(self.set_at)
                .num_seconds(),
        )
    }

    /// Gets remaining mute duration
    pub fn get_duration_remaining(&self) -> Duration {
        Duration::seconds(
            self.expire_at
                .signed_duration_since(Utc::now().naive_utc())
                .num_seconds(),
        )
    }

    /// Gets human readable formatted duration string of total mute duration
    pub fn get_human_duration(&self) -> String {
        humantime::format_duration(self.get_duration().to_std().unwrap()).to_string()
    }

    /// Gets human readable formatted duration string of remaining time
    pub fn get_human_duration_remaining(&self) -> String {
        humantime::format_duration(self.get_duration_remaining().to_std().unwrap()).to_string()
    }

    /// Gets all of a user's reminders
    pub async fn user_reminders(ctx: &Context, user_id: UserId) -> Result<Vec<Self>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        user_reminders_query(&pool, user_id).await
    }

    /// Get all reminders that have expired
    pub async fn get_expired(ctx: &Context) -> Result<Vec<Reminder>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        get_expired_query(&pool).await
    }

    /// Save a reminder to DB
    pub async fn save(&self, ctx: &Context) -> Result<Self> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        insert_query(&pool, &self).await
    }

    /// Save a reminder to DB
    pub async fn delete(&self, ctx: &Context) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        delete_query(&pool, &self).await
    }
}

async fn user_reminders_query(pool: &sqlx::PgPool, user_id: UserId) -> Result<Vec<Reminder>> {
    sqlx::query_as!(
        Reminder,
        r#"
            SELECT *
              FROM reminders
             WHERE user_id = $1
        "#,
        i64::from(user_id),
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn get_expired_query(pool: &sqlx::PgPool) -> Result<Vec<Reminder>> {
    sqlx::query_as!(
        Reminder,
        r#"
            SELECT *
              FROM reminders
             WHERE NOW() > expire_at
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn insert_query(pool: &sqlx::PgPool, reminder: &Reminder) -> Result<Reminder> {
    sqlx::query_as!(
        Reminder,
        r#"
        INSERT INTO reminders
             VALUES ($1, $2, $3, $4)
          RETURNING *
        "#,
        reminder.user_id,
        reminder.description,
        reminder.set_at,
        reminder.expire_at,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

async fn delete_query(pool: &sqlx::PgPool, reminder: &Reminder) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM reminders
              WHERE user_id = $1
                AND set_at = $2
        "#,
        reminder.user_id,
        reminder.set_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}
