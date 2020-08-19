use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::prelude::*;

#[async_trait]
pub trait GuildConfigDb {
    async fn from_msg(ctx: &Context, msg: &Message) -> Result<Option<GuildConfig>>;
    async fn from_id(ctx: &Context, guild_id: &GuildId) -> Result<Option<GuildConfig>>;

    async fn get(
        ctx: &Context,
        msg: Option<&Message>,
        guild_id: Option<&GuildId>,
    ) -> Result<Option<GuildConfig>>;

    async fn cache(&self, ctx: &Context) -> bool;
    async fn save(&self, ctx: &Context) -> Result<()>;
    fn save_bg(&self, ctx: &Context);
}

#[derive(Deserialize, Default, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct GuildConfig {
    pub id: i64,
    pub prefix: Option<String>,
    pub name: Option<String>,
    pub join_msg: Option<String>,
    pub join_react: Option<String>,
    pub leave_msg: Option<String>,
    pub msg_channel: Option<i64>,
    pub role_channel: Option<i64>,
    pub role_config: Option<serde_json::Value>,
    pub invite_guard: Option<bool>,
    pub log_msg: Option<i64>,
    pub log_mod: Option<i64>,
    pub log_member: Option<i64>,
    pub mute_role: Option<i64>,
    pub max_mention: Option<i32>,
    pub disabled_channels: Option<Vec<i64>>,
}

impl GuildConfig {
    pub fn new(id: i64) -> Self {
        GuildConfig {
            id,
            ..Default::default()
        }
    }
}

#[async_trait]
impl GuildConfigDb for GuildConfig {
    /// Gets a GuildConfig from a given message
    async fn from_msg(ctx: &Context, msg: &Message) -> Result<Option<GuildConfig>> {
        GuildConfig::get(ctx, Some(msg), None).await
    }

    /// Gets a Guildconfig from a guild id
    async fn from_id(ctx: &Context, guild_id: &GuildId) -> Result<Option<GuildConfig>> {
        GuildConfig::get(ctx, None, Some(guild_id)).await
    }

    /// Gets a GuildConfig from the cache or database
    async fn get(
        ctx: &Context,
        msg: Option<&Message>,
        guild_id: Option<&GuildId>,
    ) -> Result<Option<GuildConfig>> {
        // Return None if no guild found
        let guild_id = match guild_id.or(msg.and_then(|m| m.guild_id.as_ref())) {
            Some(id) => id,
            None => return Ok(None),
        };

        let data = ctx.data.read().await;
        let sushii_cache = data.get::<SushiiCache>().unwrap();

        // Check if exists in cache
        if sushii_cache.guilds.contains_key(&guild_id) {
            return Ok(sushii_cache
                .guilds
                .get(&guild_id)
                .map(|e| e.value().clone()));
        }

        // Not in cache, fetch from database
        let pool = data.get::<DbPool>().unwrap();
        let conf = get_guild_config_query(&pool, guild_id.0).await
            .or_else(|err| {
                // If row isn't found, create new and save
                if let Error::Sqlx(sqlx::Error::RowNotFound) = err {
                    let new_conf = GuildConfig::new(guild_id.0 as i64);
                    new_conf.save_bg(&ctx);

                    Ok(new_conf)
                } else {
                    tracing::error!(?msg, "Failed to fetch config: {}", err);
                    Err(err)
                }
            })?;

        sushii_cache.guilds.insert(*guild_id, conf.clone());
        tracing::info!(guild_id = guild_id.0, ?conf, "Cached guild config");

        Ok(Some(conf))
    }

    /// Updates config in the cache
    async fn cache(&self, ctx: &Context) -> bool {
        let data = ctx.data.read().await;
        let sushii_cache = data.get::<SushiiCache>().unwrap();

        sushii_cache.guilds.insert(GuildId(self.id as u64), self.clone())
    }

    /// Saves config to database
    async fn save(&self, ctx: &Context) -> Result<()> {
        let data = ctx.data.read().await;
        let pool = data.get::<DbPool>().unwrap();

        // Update db and cache
        upsert_config_query(self, pool).await?;
        let _ = self.cache(&ctx);

        Ok(())
    }

    /// Saves config in the background, does NOT respond with an error but does log errors
    fn save_bg(&self, ctx: &Context) {
        let conf = self.clone();
        let ctx = ctx.clone();
        tokio::spawn(async move {
            if let Err(e) = conf.save(&ctx).await {
                tracing::error!("Failed to save config in background: {}", e);
            }
        });
    }
}

async fn get_guild_config_query(pool: &sqlx::PgPool, guild_id: u64) -> Result<GuildConfig> {
    sqlx::query_as!(
        GuildConfig,
        r#"
            SELECT *
              FROM guild_configs
             WHERE id = $1
        "#,
        guild_id as i64
    )
    .fetch_one(pool)
    .await
    .map_err(Into::into)
}

async fn upsert_config_query(conf: &GuildConfig, pool: &sqlx::PgPool) -> Result<()> {
    // Bruh
    sqlx::query!(
        r#"
            INSERT INTO guild_configs
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (id)
              DO UPDATE 
                    SET prefix            = $2,
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
    .await
    .map(|_| ())
    .map_err(Into::into)
}
