use chrono::naive::NaiveDateTime;
#[cfg(not(feature = "graphql"))]
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "graphql"))]
use crate::keys::DbPool;
#[cfg(not(feature = "graphql"))]
use serenity::{model::prelude::*, prelude::*};

#[cfg(feature = "graphql")]
use juniper::GraphQLObject;

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
