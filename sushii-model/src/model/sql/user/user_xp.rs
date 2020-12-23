use serde::{Deserialize, Serialize};

#[cfg(not(feature = "graphql"))]
use crate::keys::DbPool;
#[cfg(not(feature = "graphql"))]
use serenity::{model::prelude::*, prelude::*};

#[cfg(feature = "graphql")]
use juniper::GraphQLObject;

use crate::error::{Error, Result};
use crate::model::BigInt;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
#[cfg_attr(
    feature = "graphql",
    graphql(description = "A user's XP and rank in a single guild"),
    derive(GraphQLObject)
)]
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
            let bytes = base64::decode(s)?;

            // 2 i64's
            if bytes.len() != 16 {
                return Err(Error::Sushii("Invalid cursor length (not 16 bytes)".into()));
            }

            // Convert slice to array
            let mut xp_bytes: [u8; 8] = Default::default();
            xp_bytes.copy_from_slice(&bytes[..8]);
            let mut user_id_bytes: [u8; 8] = Default::default();
            user_id_bytes.copy_from_slice(&bytes[8..]);

            // Convert byte array to i64
            let xp = i64::from_le_bytes(xp_bytes);
            let user_id = i64::from_le_bytes(user_id_bytes);

            Some((xp, user_id))
        } else {
            None
        };

        guild_top_all_time_query(pool, guild_id.0, first.0, after_bytes).await
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
