use juniper::{graphql_object, FieldResult};
use std::sync::Arc;
use sushii_model::model::{sql::UserLevel, BigInt};

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

#[graphql_object(
    context = Context,
)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    async fn rank(
        ctx: &Context,
        user_id: BigInt,
        guild_id: BigInt,
    ) -> FieldResult<Option<UserLevel>> {
        let user_level = UserLevel::from_id(&ctx.pool, user_id, guild_id).await?;

        Ok(user_level)
    }
}
