use crate::error::{Error, Result};
use crate::model::context::SushiiContext;
use crate::model::sql::guild::GuildConfig;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn cache_guild_config(ctx: &Context, msg: &Message) {
    let data = ctx.data.read().await;
    let sushii_ctx = data.get::<SushiiContext>().unwrap();

    let guild_id = match msg.guild_id {
        Some(i) => i,
        None => return,
    };

    if sushii_ctx.sushii_cache.guilds.contains_key(&guild_id) {
        return;
    }

    let conf = match cache_guild_config_query(&sushii_ctx, guild_id.0).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(?msg, "Failed to fetch config: {}", e);
            return;
        }
    };

    sushii_ctx.sushii_cache.guilds.insert(guild_id, conf);

    tracing::info!(guild_id = guild_id.0, "Cached guild config");
}

async fn cache_guild_config_query(ctx: &SushiiContext, guild_id: u64) -> Result<GuildConfig> {
    let conf_result = sqlx::query_as!(GuildConfig, r#"
            SELECT *
              FROM guild_configs
             WHERE id = $1
        "#,
        guild_id as i64)
        .fetch_one(&ctx.pool).await;

    if let Err(e) = conf_result {
        match e {
            // If not found, insert default config
            sqlx::Error::RowNotFound => {
                let pool = ctx.pool.clone();

                // Create new config in background and just return default immediately
                tokio::spawn(async move { insert_config_query(guild_id, pool) });

                return Ok(GuildConfig::new(guild_id as i64));
            },

            _ => return Err(Error::Sqlx(e)),
        }
    }

    // res is ok now
    Ok(conf_result.unwrap())
}

async fn insert_config_query(guild_id: u64, pool: sqlx::PgPool) {
    if let Err(e) = sqlx::query!(r#"
            INSERT INTO guild_configs
                VALUES ($1)
        "#,
        guild_id as i64)
        .execute(&pool)
        .await {
            tracing::error!(guild_id, "Failed to insert guild config: {}", e);
        }

    tracing::info!(guild_id, "Inserted new guild config");
}
