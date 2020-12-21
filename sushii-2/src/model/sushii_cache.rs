use serenity::model::id::GuildId;
use std::sync::Arc;

use super::sql::GuildConfig;

#[derive(Default, Clone)]
pub struct SushiiCache {
    pub guilds: Arc<dashmap::DashMap<GuildId, GuildConfig>>,
}
