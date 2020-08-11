use std::sync::Arc;
use twilight::model::channel::message::Message;

use crate::error::{Error, Result};
use crate::model::context::SushiiContext;
use crate::model::sql::guild::GuildConfig;

pub async fn cache_guild_config<'a>(msg: &Message, ctx: &Arc<SushiiContext<'a>>) {
    let guild_id = match msg.guild_id {
        Some(i) => i,
        None => return,
    };

    if ctx.sushii_cache.guilds.contains_key(&guild_id) {
        return;
    }

    let conf = match cache_guild_config_query(guild_id.0, &ctx).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(?msg, "Failed to fetch config: {}", e);
            return;
        }
    };

    ctx.sushii_cache.guilds.insert(guild_id, conf);

    tracing::info!(guild_id = guild_id.0, "Cached guild config");
}

async fn insert_config_query(guild_id: u64, pool: sqlx::PgPool) {
    if let Err(e) = sqlx::query!(r#"
            INSERT INTO guild_configs
                VALUES ($1)
        "#,
        guild_id as i64)
        .execute(&pool)
        .await {
            tracing::error!("Failed to insert guild config: {}", e);
        }
}

async fn cache_guild_config_query<'a>(guild_id: u64, ctx: &Arc<SushiiContext<'a>>) -> Result<GuildConfig> {
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

                tokio::spawn(async move { insert_config_query(guild_id, pool) });

                return Ok(GuildConfig::new(guild_id as i64));
            },

            _ => return Err(Error::Sqlx(e)),
        }
    }

    // res is ok now
    Ok(conf_result.unwrap())
}
