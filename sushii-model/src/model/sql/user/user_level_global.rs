use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::types::Decimal;

use crate::error::Result;
use crate::keys::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct UserLevelGlobal {
    pub xp: Option<Decimal>,
}

impl UserLevelGlobal {
    pub async fn from_id(ctx: &Context, user_id: UserId) -> Result<Option<UserLevelGlobal>> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        from_id_query(&pool, user_id).await
    }
}

async fn from_id_query(pool: &sqlx::PgPool, user_id: UserId) -> Result<Option<UserLevelGlobal>> {
    sqlx::query_as!(
        UserLevelGlobal,
        r#"
            SELECT SUM(msg_all_time) AS xp
              FROM user_levels
             WHERE user_id = $1
        "#,
        i64::from(user_id),
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}
