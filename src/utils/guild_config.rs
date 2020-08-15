use crate::error::{Error, Result};
use crate::keys::*;
use crate::model::sql::guild::GuildConfig;
use serenity::model::prelude::*;
use serenity::prelude::*;

// TODO: Try reducing clones? Not entirely sure about multiple clones for every message
// Returns None even if something failed, not just that the config wasn't found.
pub async fn get_cached_guild_config(ctx: &Context, msg: &Message) -> Option<GuildConfig> {
    let data = ctx.data.read().await;
    let sushii_cache = data.get::<SushiiCache>().unwrap();

    let guild_id = msg.guild_id?;

    if sushii_cache.guilds.contains_key(&guild_id) {
        return sushii_cache
            .guilds
            .get(&guild_id)
            .map(|e| e.value().clone());
    }

    let pool = data.get::<DbPool>().unwrap();
    let conf = match cache_guild_config_query(&pool, guild_id.0).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(?msg, "Failed to fetch config: {}", e);
            return None;
        }
    };

    // Log before insert since insert takes ownership
    tracing::info!(guild_id = guild_id.0, ?conf, "Cached guild config");
    sushii_cache
        .guilds
        .insert(guild_id, conf.clone());

    Some(conf)
}

async fn cache_guild_config_query(pool: &sqlx::PgPool, guild_id: u64) -> Result<GuildConfig> {
    let conf_result = sqlx::query_as!(
        GuildConfig,
        r#"
            SELECT *
              FROM guild_configs
             WHERE id = $1
        "#,
        guild_id as i64
    )
    .fetch_one(pool)
    .await;

    if let Err(e) = conf_result {
        match e {
            // If not found, insert default config
            sqlx::Error::RowNotFound => {
                let pool = pool.clone();

                // Create new config in background and just return default immediately
                tokio::spawn(async move { insert_default_config_query(guild_id, pool) });

                return Ok(GuildConfig::new(guild_id as i64));
            }

            _ => return Err(Error::Sqlx(e)),
        }
    }

    // res is ok now
    Ok(conf_result.unwrap())
}

async fn insert_default_config_query(guild_id: u64, pool: sqlx::PgPool) {
    if let Err(e) = sqlx::query!(
        r#"
            INSERT INTO guild_configs
                 VALUES ($1)
        "#,
        guild_id as i64
    )
    .execute(&pool)
    .await
    {
        tracing::error!(guild_id, "Failed to insert guild config: {}", e);
    }

    tracing::info!(guild_id, "Inserted new guild config");
}

pub async fn upsert_config(ctx: &Context, conf: &GuildConfig) -> Result<()> {
    let data = ctx.data.read().await;
    let pool = data.get::<DbPool>().unwrap();

    upsert_config_query(conf, pool).await
}

async fn upsert_config_query(conf: &GuildConfig, pool: &sqlx::PgPool) -> Result<()> {
    // Bruh
    sqlx::query!(
        r#"
            INSERT INTO guild_configs
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (id)
              DO UPDATE 
                    SET 
                        prefix            = $2,
                        name              = $3,
                        join_msg          = $4,
                        join_react        = $5,
                        leave_msg         = $6,
                        msg_channel       = $7,
                        role_channel      = $8,
                        role_config       = $9,
                        invite_guard      = $10,
                        log_msg           = $11,
                        log_mod           = $12,
                        log_member        = $13,
                        mute_role         = $14,
                        max_mention       = $15,
                        disabled_channels = $16
        "#,
        conf.id as i64,
        conf.prefix,
        conf.name,
        conf.join_msg,
        conf.join_react,
        conf.leave_msg,
        conf.msg_channel,
        conf.role_channel,
        conf.role_config,
        conf.invite_guard,
        conf.log_msg,
        conf.log_mod,
        conf.log_member,
        conf.mute_role,
        conf.max_mention,
        // Needs &[i64] instead of Vec<i64>
        conf.disabled_channels.as_deref(),
    )
    .execute(pool)
    .await?;

    tracing::info!(?conf, "Upsert guild config");

    Ok(())
}
