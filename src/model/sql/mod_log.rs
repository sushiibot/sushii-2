use chrono::naive::NaiveDateTime;
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;

#[async_trait]
pub trait ModLogEntryDb {
    async fn get_pending_entry(
        ctx: &Context,
        mod_action: &str,
        guild_id: u64,
        target_id: u64,
    ) -> Result<Option<ModLogEntry>>;

    async fn get_user_entries(
        ctx: &Context,
        guild_id: u64,
        user_id: u64,
    ) -> Result<Vec<ModLogEntry>>;

    async fn get_range_entries(
        ctx: &Context,
        guild_id: u64,
        start: u64,
        end: u64,
    ) -> Result<Vec<ModLogEntry>>;

    async fn get_latest(ctx: &Context, guild_id: u64) -> Result<Option<ModLogEntry>>;

    async fn save(&self, ctx: &Context) -> Result<ModLogEntry>;
    async fn delete(&self, ctx: &Context) -> Result<()>;
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct ModLogEntry {
    pub guild_id: i64,
    pub case_id: i64,

    /// Metadata
    pub action: String,
    pub action_time: NaiveDateTime,
    pub pending: bool,

    /// Target user info
    pub user_id: i64,
    pub user_tag: String,

    /// Moderator id
    pub executor_id: Option<i64>,
    pub reason: Option<String>,
    pub msg_id: Option<i64>,
}

impl ModLogEntry {
    /// Creates a new ModLogEntry, with the case_id of -1
    pub fn new(action: &str, pending: bool, guild_id: u64, user: &User) -> Self {
        ModLogEntry {
            guild_id: guild_id as i64,
            // This is temporary as we get the actual case_id when inserting into db
            case_id: -1,
            action: action.to_string(),
            action_time: Utc::now().naive_local(),
            pending,
            user_id: user.id.0 as i64,
            user_tag: user.tag(),
            executor_id: None,
            reason: None,
            msg_id: None,
        }
    }
}

#[async_trait]
impl ModLogEntryDb for ModLogEntry {
    /// Fetches a pending ModLogEntry. Returns Err if something failed, None if
    /// not found. This is because we want to stop if something failed, but
    /// continue if not found, so a not found "error" should not be treated as
    /// an error.
    async fn get_pending_entry(
        ctx: &Context,
        mod_action: &str,
        guild_id: u64,
        target_id: u64,
    ) -> Result<Option<Self>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        get_pending_entry_query(pool, mod_action, guild_id, target_id)
            .await
            .map_err(|err| {
                // Do not need to handle row not found error since using fetch_optional
                tracing::error!(
                    mod_action,
                    guild_id,
                    target_id,
                    "Failed to query pending mod log entry: {}",
                    err
                );

                err
            })
    }

    async fn get_user_entries(
        ctx: &Context,
        guild_id: u64,
        user_id: u64,
    ) -> Result<Vec<ModLogEntry>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        get_user_entries_query(pool, guild_id, user_id).await
    }

    async fn get_range_entries(
        ctx: &Context,
        guild_id: u64,
        start: u64,
        end: u64,
    ) -> Result<Vec<Self>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        // Enforce start / end ordering
        let start = std::cmp::min(start, end);
        let end = std::cmp::max(start, end);

        get_range_entries_query(pool, guild_id, start, end).await
    }

    async fn get_latest(ctx: &Context, guild_id: u64) -> Result<Option<ModLogEntry>> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        get_latest_query(pool, guild_id).await
    }

    /// Saves a ModLogEntry to the database. Returns a new one from the database
    /// with a valid case_id
    async fn save(&self, ctx: &Context) -> Result<Self> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        add_mod_action_query(&pool, self).await
    }

    async fn delete(&self, ctx: &Context) -> Result<()> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        delete_mod_action_query(&pool, self).await
    }
}

async fn get_pending_entry_query(
    pool: &sqlx::PgPool,
    mod_action: &str,
    guild_id: u64,
    target_id: u64,
) -> Result<Option<ModLogEntry>> {
    sqlx::query_as!(
        ModLogEntry,
        r#"
            SELECT *
              FROM mod_logs
             WHERE guild_id = $1
               AND user_id = $2
               AND action = $3
               AND pending = true
        "#,
        guild_id as i64,
        target_id as i64,
        mod_action
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn get_user_entries_query(
    pool: &sqlx::PgPool,
    guild_id: u64,
    user_id: u64,
) -> Result<Vec<ModLogEntry>> {
    sqlx::query_as!(
        ModLogEntry,
        r#"
            SELECT *
              FROM mod_logs
             WHERE guild_id = $1
               AND user_id >= $2
        "#,
        guild_id as i64,
        user_id as i64,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn get_range_entries_query(
    pool: &sqlx::PgPool,
    guild_id: u64,
    start: u64,
    end: u64,
) -> Result<Vec<ModLogEntry>> {
    sqlx::query_as!(
        ModLogEntry,
        r#"
            SELECT *
              FROM mod_logs
             WHERE guild_id = $1
               AND case_id >= $2
               AND case_id <= $3
        "#,
        guild_id as i64,
        start as i64,
        end as i64,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn get_latest_query(pool: &sqlx::PgPool, guild_id: u64) -> Result<Option<ModLogEntry>> {
    sqlx::query_as!(
        ModLogEntry,
        r#"
              SELECT *
                FROM mod_logs
               WHERE guild_id = $1
            ORDER BY case_id DESC
               LIMIT 1
        "#,
        guild_id as i64,
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn add_mod_action_query(pool: &sqlx::PgPool, entry: &ModLogEntry) -> Result<ModLogEntry> {
    sqlx::query_as!(
        ModLogEntry,
        r#"
            INSERT INTO mod_logs
                 VALUES ($1, (
                            SELECT COALESCE(MAX(case_id) + 1, 0)
                              FROM mod_logs
                             WHERE guild_id = $1
                        ), $2, $3, $4, $5, $6, $7, $8, $9)
              RETURNING *
        "#,
        entry.guild_id,
        // Don't include this since we fetch the new one in the query
        // entry.case_id: i64,
        entry.action,
        entry.action_time,
        entry.pending,
        entry.user_id,
        entry.user_tag,
        entry.executor_id,
        entry.reason,
        entry.msg_id
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

async fn delete_mod_action_query(pool: &sqlx::PgPool, entry: &ModLogEntry) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM mod_logs
                  WHERE guild_id = $1
                    AND case_id = $2
                    AND user_id = $3
        "#,
        entry.guild_id,
        entry.case_id,
        entry.user_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[test]
fn new_mod_log_entry() {
    let entry = ModLogEntry::new("ban", false, 1234, &User::default());

    // https://docs.rs/serenity/0.9.0-rc.0/src/serenity/model/user.rs.html#409-426
    assert_eq!(entry.user_id, 210);
    assert_eq!(entry.user_tag, "test#1432");
    assert_eq!(entry.case_id, -1);
}
