use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;

use sushii_model::model::sql::GuildConfig;
use twilight_model::id::GuildId;

type GuildConfigMap = DashMap<GuildId, Arc<GuildConfig>>;

#[derive(Debug, Clone)]
pub struct GuildConfigCache {
    cache: Arc<GuildConfigMap>,
}

impl GuildConfigCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    pub async fn get(&self, pool: &sqlx::PgPool, guild_id: GuildId) -> Result<Arc<GuildConfig>> {
        // Fetch from db if not in cache
        if !self.cache.contains_key(&guild_id) {
            let conf = GuildConfig::from_id_db(pool, guild_id.0)
                .await?
                .unwrap_or_else(|| GuildConfig::new(guild_id.0 as i64));
            self.cache.insert(guild_id, Arc::new(conf));
        }

        let conf = self
            .cache
            .get(&guild_id)
            .expect("Guild conf missing from cache somehow")
            .clone();

        Ok(conf)
    }
}
