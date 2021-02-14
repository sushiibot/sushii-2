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
    graphql(description = "A cached Discord guild"),
    derive(GraphQLObject)
)]
pub struct CachedGuild {
    pub id: BigInt,
    pub name: String,
    pub member_count: BigInt,
    pub icon_url: Option<String>,
    pub features: String,
    pub splash_url: Option<String>,
    pub banner_url: Option<String>,
}

impl CachedGuild {
    #[cfg(feature = "graphql")]
    pub async fn from_id(pool: &sqlx::PgPool, guild_id: BigInt) -> Result<Option<Self>> {
        from_id_query(pool, guild_id.0).await
    }

    /// Updates guild
    #[cfg(not(feature = "graphql"))]
    pub async fn update(ctx: &Context, guild: &Guild) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        update_query(&pool, guild).await
    }

    /// Updates guild from a partial guild obj
    #[cfg(not(feature = "graphql"))]
    pub async fn update_from_partial(ctx: &Context, partial_guild: &PartialGuild) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        // update_guild dispatch runs after cache is updated, so this should include new info
        if let Some(guild) = partial_guild.id.to_guild_cached(&ctx).await {
            update_query(&pool, &guild).await
        } else {
            tracing::warn!("Cached guild not found when updating");

            Ok(())
        }
    }
}

#[cfg(feature = "graphql")]
async fn from_id_query(pool: &sqlx::PgPool, user_id: i64) -> Result<Option<CachedGuild>> {
    sqlx::query_as!(
        CachedGuild,
        r#"
            SELECT id as "id: BigInt",
                   name,
                   member_count as "member_count: BigInt",
                   icon_url,
                   features,
                   splash_url,
                   banner_url
              FROM app_public.cached_guilds
             WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

#[cfg(not(feature = "graphql"))]
async fn update_query(pool: &sqlx::PgPool, guild: &Guild) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO app_public.cached_guilds
             VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (id)
          DO UPDATE
                SET name = $2,
                    member_count = $3,
                    icon_url = $4,
                    features = $5,
                    splash_url = $6,
                    banner_url = $7
        "#,
        i64::from(guild.id),
        guild.name,
        guild.member_count as i64,
        guild.icon_url(),
        guild.features.join(", "),
        guild.splash_url(),
        guild.banner_url(),
    )
    .execute(pool)
    .await?;

    Ok(())
}
