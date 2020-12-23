use juniper::{graphql_object, FieldResult};
use std::sync::Arc;
use sushii_model::model::{
    sql::{UserGuildXP, UserLevel, UserLevelRanked},
    BigInt,
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

    async fn guild_ranks(
        ctx: &Context,
        guild_id: BigInt,
        first: BigInt,
        after: Option<String>,
    ) -> FieldResult<UserGuildXPConnection> {
        let user_level_ranked =
            UserGuildXP::guild_top_all_time(&ctx.pool, guild_id, first, after).await?;

        let con = UserGuildXPConnection {
            edges: user_level_ranked
                .into_iter()
                .map(|node| {
                    let cursor = base64::encode(
                        [node.xp.0.to_le_bytes(), node.user_id.0.to_le_bytes()].concat(),
                    );

                    UserGuildXPEdge { node, cursor }
                })
                .collect(),
            page_info: PageInfo {
                has_previous_page: false,
                has_next_page: true,
                start_cursor: "".into(),
                end_cursor: "".into(),
            },
        };

        Ok(con)
    }
}

relay_connection!(UserGuildXPConnection, UserGuildXPEdge, UserGuildXP, Context);
