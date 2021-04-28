use chrono::naive::NaiveDateTime;
use chrono::{offset::Utc, Duration};
use serde::{Deserialize, Serialize};

use crate::keys::DbPool;
use serenity::{model::prelude::*, prelude::*};

use crate::error::Result;
use crate::model::BigInt;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct CachedUser {
    pub id: BigInt,
    pub avatar_url: String,
    pub name: String,
    pub discriminator: i32,
    pub last_checked: NaiveDateTime,
}

impl CachedUser {
    /// Updates user
    pub async fn update(ctx: &Context, user: &User) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        update_query(&pool, user).await
    }
}

async fn from_id_query(pool: &sqlx::PgPool, user_id: i64) -> Result<Option<CachedUser>> {
    sqlx::query_as!(
        CachedUser,
        r#"
            SELECT id as "id: BigInt",
                   avatar_url,
                   name,
                   discriminator,
                   last_checked
              FROM app_public.cached_users
             WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

async fn update_query(pool: &sqlx::PgPool, user: &User) -> Result<()> {
    let cached_user = from_id_query(pool, user.id.0 as i64).await?;

    if let Some(cached_user) = cached_user {
        let now = Utc::now().naive_utc();

        // If not yet 1 day since last check, skip
        if now < (cached_user.last_checked + Duration::days(1)) {
            return Ok(());
        }
    }

    sqlx::query!(
        r#"
        INSERT INTO app_public.cached_users
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
