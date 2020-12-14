use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::types::Json;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct SavedMessage {
    pub message_id: i64,
    pub author_id: i64,
    pub channel_id: i64,
    pub guild_id: i64,
    pub created: NaiveDateTime,
    pub content: String,
    pub msg: Json<Message>,
}

impl SavedMessage {
    pub fn from_msg(msg: &Message) -> Option<Self> {
        Some(Self {
            message_id: i64::from(msg.id),
            author_id: i64::from(msg.author.id),
            channel_id: i64::from(msg.channel_id),
            guild_id: msg.guild_id.map(|id| i64::from(id))?,
            created: msg.timestamp.naive_utc(),
            content: msg.content.clone(),
            msg: Json(msg.clone()),
        })
    }

    pub async fn save(self, ctx: &Context) -> Result<Self> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            SavedMessage,
            r#"
            INSERT INTO messages (
                            message_id,
                            author_id,
                            channel_id,
                            guild_id,
                            created,
                            content,
                            msg)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (message_id)
              DO UPDATE
                    SET author_id = $2,
                        channel_id = $3,
                        guild_id = $4,
                        created = $5,
                        content = $6,
                        msg = $7
            "#,
            self.message_id,
            self.author_id,
            self.channel_id,
            self.guild_id,
            self.created,
            self.content,
            self.msg as _, // Converts to serde_json::Value I think
        )
        .execute(&pool)
        .await?;

        // Instead of RETURNING * just return self
        Ok(self)
    }

    pub async fn from_id(ctx: &Context, message_id: MessageId) -> Result<Option<Self>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        sqlx::query_as!(
            SavedMessage,
            r#"
                SELECT message_id,
                       author_id,
                       channel_id,
                       guild_id,
                       created,
                       content,
                       msg as "msg: Json<Message>"
                  FROM messages
                 WHERE message_id = $1
            "#,
            i64::from(message_id),
        )
        .fetch_optional(&pool)
        .await
        .map_err(Into::into)
    }

    pub async fn prune_old(ctx: &Context, channel_id: ChannelId) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        // Only keep last 100 messages
        sqlx::query!(
            r#"
                DELETE FROM messages
                      WHERE channel_id = $1
                            AND ctid NOT IN (
                                  SELECT ctid
                                    FROM messages
                                ORDER BY created DESC
                                   LIMIT 100
                            )
            "#,
            i64::from(channel_id),
        )
        .execute(&pool)
        .await?;

        Ok(())
    }
}
