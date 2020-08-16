use chrono::offset::Utc;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;
use crate::model::sql::mod_log::ModLogEntry;

pub async fn get_pending_entries(
    ctx: &Context,
    mod_action: &str,
    guild_id: u64,
    target_id: u64,
) -> Option<ModLogEntry> {
    let data = ctx.data.read().await;
    let pool = data.get::<DbPool>().unwrap();

    get_pending_entries_query(pool, mod_action, guild_id, target_id).await
}

pub async fn add_mod_action(
    ctx: &Context,
    action: &str,
    pending: bool,
    guild_id: u64,
    user: &User,
) -> Result<ModLogEntry> {
    let new_entry = ModLogEntry {
        guild_id: guild_id as i64,
        // This is temporary as we can just get the actual case_id when inserting into db
        case_id: -1,
        action: action.to_string(),
        action_time: Utc::now().naive_local(),
        pending,
        user_id: user.id.0 as i64,
        user_tag: user.tag(),
        executor_id: None,
        reason: None,
        msg_id: None,
    };

    let data = ctx.data.read().await;
    let pool = data.get::<DbPool>().unwrap();

    add_mod_action_query(&pool, new_entry).await
}

async fn add_mod_action_query(pool: &sqlx::PgPool, entry: ModLogEntry) -> Result<ModLogEntry> {
    sqlx::query_as!(
        ModLogEntry,
        r#"
            INSERT INTO mod_logs
                 VALUES ($1, (
                            SELECT MAX(case_id) + 1
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

async fn get_pending_entries_query(
    pool: &sqlx::PgPool,
    mod_action: &str,
    guild_id: u64,
    target_id: u64,
) -> Option<ModLogEntry> {
    let res = sqlx::query_as!(
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
    .fetch_one(pool)
    .await;

    if let Err(e) = res {
        match e {
            // If not found return none
            sqlx::Error::RowNotFound => return None,
            _ => {
                tracing::error!(
                    mod_action,
                    guild_id,
                    target_id,
                    "Failed to query pending mod log entry: {}",
                    e
                );

                return None;
            }
        }
    }

    res.ok()
}
