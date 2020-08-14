use serenity::model::id::GuildId;
use std::sync::Arc;

use super::sql::guild;

#[derive(Default, Clone)]
pub struct SushiiCache {
    pub guilds: Arc<dashmap::DashMap<GuildId, guild::GuildConfig>>,
}
