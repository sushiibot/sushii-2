use juniper::{graphql_object, FieldResult};
use std::sync::Arc;
use sushii_model::{
    model::{
        sql::{UserLevel, UserLevelRanked, UserXP},
        BigInt,
    },
    Error,
};

use crate::{relay::PageInfo, relay_connection};

#[derive(Clone)]
pub struct Context {
    pool: Arc<sqlx::PgPool>,
}

impl Context {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    async fn level(
        ctx: &Context,
        user_id: BigInt,
        guild_id: BigInt,
    ) -> FieldResult<Option<UserLevel>> {
        let user_level = UserLevel::from_id(&ctx.pool, user_id, guild_id).await?;

        Ok(user_level)
    }

    async fn rank(
        ctx: &Context,
        user_id: BigInt,
        guild_id: BigInt,
    ) -> FieldResult<Option<UserLevelRanked>> {
        let user_level_ranked = UserLevelRanked::from_id(&ctx.pool, user_id, guild_id).await?;

        Ok(user_level_ranked)
    }

    async fn user_guild_xp_connection(
        ctx: &Context,
        guild_id: BigInt,
        first: BigInt,
        after: Option<String>,
    ) -> FieldResult<UserXPConnection> {
        let user_level_ranked =
            UserXP::guild_top_all_time(&ctx.pool, guild_id, first, after).await?;

        let edges: Vec<UserXPEdge> = user_level_ranked
            .into_iter()
            .map(|node| {
                // Cursor [XP, user_id] bytes to base64
                let cursor = base64::encode(
                    [node.xp.0.to_le_bytes(), node.user_id.0.to_le_bytes()].concat(),
                );

                UserXPEdge { node, cursor }
            })
            .collect();

        let page_info = PageInfo {
            // No backwards pagination support for now
            has_previous_page: false,
            has_next_page: true,
            start_cursor: edges
                .first()
                .map(|e| e.cursor.clone())
                .ok_or_else(|| Error::Sushii("No data was returned".into()))?,
            end_cursor: edges
                .last()
                .map(|e| e.cursor.clone())
                .ok_or_else(|| Error::Sushii("No data was returned".into()))?,
        };

        Ok(UserXPConnection { edges, page_info })
    }
}

relay_connection!(UserXPConnection, UserXPEdge, UserXP, Context);
