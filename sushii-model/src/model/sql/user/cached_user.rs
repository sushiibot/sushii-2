use chrono::naive::NaiveDateTime;
#[cfg(not(feature = "graphql"))]
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "graphql"))]
use crate::keys::DbPool;
#[cfg(not(feature = "graphql"))]
use serenity::{model::prelude::*, prelude::*};

#[cfg(feature = "graphql")]
use dataloader::{non_cached::Loader, BatchFn};
#[cfg(feature = "graphql")]
use juniper::GraphQLObject;
#[cfg(feature = "graphql")]
use serenity::async_trait;
#[cfg(feature = "graphql")]
use std::{collections::HashMap, sync::Arc};

use crate::error::Result;
use crate::model::BigInt;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
#[cfg_attr(
    feature = "graphql",
    graphql(description = "A cached Discord user"),
    derive(GraphQLObject)
)]
pub struct CachedUser {
    pub id: BigInt,
    pub avatar_url: String,
    pub name: String,
    pub discriminator: i32,
    pub last_checked: NaiveDateTime,
}

impl CachedUser {
    #[cfg(feature = "graphql")]
    pub async fn from_id(pool: &sqlx::PgPool, user_id: BigInt) -> Result<Option<Self>> {
        from_id_query(pool, user_id.0).await
    }

    /// Gets multiple cached users by a list of user IDs
    #[cfg(feature = "graphql")]
    pub async fn from_ids(pool: &sqlx::PgPool, user_ids: &[i64]) -> Result<Vec<Self>> {
        from_ids_query(pool, user_ids).await
    }

    /// Updates user
    #[cfg(not(feature = "graphql"))]
    pub async fn update(ctx: &Context, user: &User) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        update_query(&pool, user).await
    }
}

#[cfg(feature = "graphql")]
async fn from_id_query(pool: &sqlx::PgPool, user_id: i64) -> Result<Option<CachedUser>> {
    sqlx::query_as!(
        CachedUser,
        r#"
            SELECT id as "id: BigInt",
                   avatar_url,
                   name,
                   discriminator,
                   last_checked
              FROM cached_users
             WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

#[cfg(feature = "graphql")]
async fn from_ids_query(pool: &sqlx::PgPool, user_ids: &[i64]) -> Result<Vec<CachedUser>> {
    sqlx::query_as!(
        CachedUser,
        r#"
            SELECT id as "id: BigInt",
                   avatar_url,
                   name,
                   discriminator,
                   last_checked
              FROM cached_users
             WHERE id = ANY($1)
        "#,
        user_ids
    )
    .fetch_all(pool)
    .await
    .map_err(Into::into)
}

#[cfg(not(feature = "graphql"))]
async fn update_query(pool: &sqlx::PgPool, user: &User) -> Result<()> {
    // look for entries older than 1 day
    let rec = sqlx::query!(
        r#"
            SELECT COUNT(*) as "count!"
              FROM cached_users
             WHERE last_checked > NOW() - INTERVAL '1 DAY'
               AND id = $1
        "#,
        i64::from(user.id)
    )
    .fetch_one(pool)
    .await?;

    // Updated within a day, skip
    // 1 if last_checked is within 1 day
    if rec.count == 1 {
        return Ok(());
    }

    sqlx::query!(
        r#"
        INSERT INTO cached_users
             VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (id)
          DO UPDATE
                SET avatar_url = $2,
                    name = $3,
                    discriminator = $4,
                    last_checked = $5
        "#,
        i64::from(user.id),
        user.face(),
        user.name,
        user.discriminator as i32,
        Utc::now().naive_utc()
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "graphql")]
pub struct CachedUserBatcher {
    pub pool: Arc<sqlx::PgPool>,
}

#[cfg(feature = "graphql")]
impl CachedUserBatcher {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }
}

#[cfg(feature = "graphql")]
#[async_trait]
impl BatchFn<i64, Option<CachedUser>> for CachedUserBatcher {
    // A hashmap is used, as we need to return an array which maps each original key to a CachedUser.
    async fn load(&mut self, keys: &[i64]) -> HashMap<i64, Option<CachedUser>> {
        let mut keys_map: HashMap<i64, Option<CachedUser>> =
            keys.into_iter().map(|id| (*id, None)).collect();

        let map: HashMap<i64, Option<CachedUser>> =
            match CachedUser::from_ids(&self.pool, keys).await {
                Ok(v) => {
                    // Overwrite default missing keys with found Some(values)
                    keys_map.extend(v.into_iter().map(|u| (u.id.0, Some(u))));

                    keys_map
                }
                Err(e) => {
                    tracing::warn!("Error batch loading cached users: {}", e);
                    // if empty, just throw out errors and use None
                    // kinda ugly workaround since we can't return a Result<> directly
                    // Result won't work since Clone on error is required, but
                    // inner Error types don't derive clone
                    keys_map
                }
            };

        map
    }
}

#[cfg(feature = "graphql")]
pub type CachedUserLoader = Loader<i64, Option<CachedUser>, CachedUserBatcher>;
