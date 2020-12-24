use serde::{Deserialize, Serialize};

#[cfg(not(feature = "graphql"))]
use crate::keys::DbPool;
#[cfg(not(feature = "graphql"))]
use serenity::{model::prelude::*, prelude::*};

#[cfg(feature = "graphql")]
use crate::{
    cursor::decode_cursor,
    model::{juniper::Context, sql::CachedUser},
};
#[cfg(feature = "graphql")]
use juniper::{graphql_object, FieldResult, GraphQLObject};

use crate::error::Result;
use crate::model::BigInt;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct UserXP {
    pub user_id: BigInt,
    /// Guild ID or None if global
    pub guild_id: Option<BigInt>,
    /// User XP in a time period
    pub xp: BigInt,
}

impl UserXP {
    /// Get guild all time ranks
    #[cfg(feature = "graphql")]
    pub async fn guild_top_all_time(
        pool: &sqlx::PgPool,
        guild_id: BigInt,
        first: BigInt,
        after: Option<String>,
    ) -> Result<(BigInt, Vec<UserXP>)> {
        let after_bytes = if let Some(s) = after {
            Some(decode_cursor(&s)?)
        } else {
            None
        };

        guild_top_all_time_query(pool, guild_id.0, first.0, after_bytes).await
    }

    /// Get global all time ranks
    #[cfg(feature = "graphql")]
    pub async fn global_top_all_time(
        pool: &sqlx::PgPool,
        first: BigInt,
        after: Option<String>,
    ) -> Result<(BigInt, Vec<UserXP>)> {
        let after_bytes = if let Some(s) = after {
            Some(decode_cursor(&s)?)
        } else {
            None
        };

        global_top_all_time_query(pool, first.0, after_bytes).await
    }
}

#[cfg(feature = "graphql")]
#[graphql_object(
    context = Context,
    description = "User XP in a given timeframe and given scope (guild or global)"
)]
impl UserXP {
    // Gotta do this for each field
    // https://github.com/graphql-rust/juniper/issues/553
    fn user_id(&self) -> BigInt {
        self.user_id
    }

    fn guild_id(&self) -> Option<BigInt> {
        self.guild_id
    }

    fn xp(&self) -> BigInt {
        self.xp
    }

    async fn user(ctx: &Context) -> FieldResult<Option<CachedUser>> {
        CachedUser::from_id(&ctx.pool, self.user_id)
            .await
            .map_err(Into::into)
    }
}

async fn guild_top_all_time_query(
    pool: &sqlx::PgPool,
    guild_id: i64,
    first: i64,
    after: Option<(i64, i64)>,
) -> Result<(BigInt, Vec<UserXP>)> {
    let (total, users) = tokio::join!(
        sqlx::query!(
            r#"
            SELECT COUNT(*) as "total!: BigInt"
              FROM user_levels
             WHERE guild_id = $1
            "#,
            guild_id,
        )
        .fetch_one(pool),
        sqlx::query_as!(
            UserXP,
            // Force guild_id to be nullable since we use None for global XP
            r#"
                SELECT user_id as "user_id: BigInt",
                       guild_id as "guild_id?: BigInt",
                       msg_all_time as "xp: BigInt"
                FROM user_levels
                WHERE guild_id = $1
                AND (msg_all_time < $2 OR $2 IS NULL)
                AND (user_id < $3 OR $3 IS NULL)
            ORDER BY "xp: BigInt" DESC,
                    "user_id: BigInt" DESC
                LIMIT $4
            "#,
            guild_id,
            after.map(|a| a.0), // xp
            after.map(|a| a.1), // user id
            first,
        )
        .fetch_all(pool)
    );

    Ok((total?.total, users?))
}

async fn global_top_all_time_query(
    pool: &sqlx::PgPool,
    first: i64,
    after: Option<(i64, i64)>,
) -> Result<(BigInt, Vec<UserXP>)> {
    let (total, users) = tokio::join!(
        sqlx::query!(
            r#"
              SELECT COUNT(*) as "total!: BigInt"
                FROM user_levels
            GROUP BY user_id
            "#,
        )
        .fetch_one(pool),
        sqlx::query_as!(
            UserXP,
            r#"
                SELECT user_id as "user_id: BigInt",
                       NULL as "guild_id?: BigInt",
                       SUM(msg_all_time) AS "xp!: BigInt"
                FROM user_levels
                WHERE (msg_all_time < $1 OR $1 IS NULL)
                  AND (user_id < $2 OR $2 IS NULL)
             GROUP BY user_id
            ORDER BY "xp!: BigInt" DESC,
                     "user_id: BigInt" DESC
                LIMIT $3
            "#,
            after.map(|a| a.0), // xp
            after.map(|a| a.1), // user id
            first,
        )
        .fetch_all(pool)
    );

    Ok((total?.total, users?))
}
