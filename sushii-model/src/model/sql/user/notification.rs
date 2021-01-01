use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug, Default)]
pub struct Notification {
    pub user_id: i64,
    pub guild_id: i64,
    pub keyword: String,
}

impl Notification {
    pub fn new(user_id: UserId, guild_id: Option<GuildId>, keyword: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            guild_id: guild_id.map_or(0, Into::into),
            keyword: keyword.into(),
        }
    }

    /// Gets a single user's notification
    pub async fn user_notification(ctx: &Context, user_id: UserId, keyword: &str) -> Result<Option<Self>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        user_notification_query(&pool, user_id, keyword).await
    }

    /// Gets all of a user's notifications
    pub async fn user_notifications(ctx: &Context, user_id: UserId) -> Result<Vec<Self>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        user_notifications_query(&pool, user_id).await
    }

    /// Get all notifications that are triggered by a given message
    pub async fn get_matching(ctx: &Context, guild_id: GuildId, text: &str) -> Result<Vec<Notification>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        get_matching_query(&pool, guild_id, text).await
    }

    /// Save a notification to DB
    pub async fn save(&self, ctx: &Context) -> Result<Notification> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        insert_query(&pool, &self).await
    }
}

async fn user_notification_query(
    pool: &sqlx::PgPool,
    user_id: UserId,
    keyword: &str,
) -> Result<Option<Notification>> {
    sqlx::query_as!(
        Notification,
        r#"
            SELECT *
              FROM notifications
             WHERE user_id = $1
               AND keyword = $2
        "#,
        i64::from(user_id),
        keyword
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn user_notifications_query(
    pool: &sqlx::PgPool,
    user_id: UserId,
) -> Result<Vec<Notification>> {
    sqlx::query_as!(
        Notification,
        r#"
            SELECT *
              FROM notifications
             WHERE user_id = $1
        "#,
        i64::from(user_id),
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn get_matching_query(
    pool: &sqlx::PgPool,
    guild_id: GuildId,
    text: &str,
) -> Result<Vec<Notification>> {
    sqlx::query_as!(
        Notification,
        r#"
            WITH words(word) AS (
                SELECT s
                  FROM regexp_split_to_table(lower($2), '[^[:alnum:]]+') s
                 WHERE s <> ''
            )
            SELECT notifications.*
              FROM notifications, words
             WHERE (guild_id = $1 OR guild_id = 0)
               AND LOWER(keyword) = word
        "#,
        i64::from(guild_id),
        text,
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

async fn insert_query(pool: &sqlx::PgPool, notification: &Notification) -> Result<Notification> {
    sqlx::query_as!(
        Notification,
        r#"
        INSERT INTO notifications (user_id, guild_id, keyword)
             VALUES ($1, $2, $3)
          RETURNING *
        "#,
        notification.user_id,
        notification.guild_id,
        notification.keyword,
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}
