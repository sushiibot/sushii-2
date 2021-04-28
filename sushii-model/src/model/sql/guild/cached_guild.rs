use serde::{Deserialize, Serialize};

#[cfg(not(feature = "graphql"))]
use crate::keys::DbPool;
#[cfg(not(feature = "graphql"))]
use serenity::{model::prelude::*, prelude::*};

use crate::error::Result;
use crate::model::BigInt;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct CachedGuild {
    pub id: BigInt,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub banner: Option<String>,
    pub features: Vec<String>,
}

impl CachedGuild {
    /// Updates guild
    pub async fn update(ctx: &Context, guild: &Guild) -> Result<()> {
        let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();

        update_query(&pool, guild).await
    }

    /// Updates guild from a partial guild obj
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

#[cfg(not(feature = "graphql"))]
async fn update_query(pool: &sqlx::PgPool, guild: &Guild) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO app_public.cached_guilds (id, name, icon, splash, banner, features)
             VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (id)
          DO UPDATE
                SET name = $2,
                    icon = $3,
                    splash = $4,
                    banner = $5,
                    features = $6
        "#,
        i64::from(guild.id),
        guild.name,
        guild.icon,
        guild.splash,
        guild.banner,
        &guild.features,
    )
    .execute(pool)
    .await?;

    Ok(())
}
