use std::sync::Arc;
use twilight::model::id::GuildId;

use super::sql::guild;

#[derive(Default, Clone)]
pub struct SushiiCache {
    pub guilds: Arc<dashmap::DashMap<GuildId, guild::GuildConfig>>,
}
