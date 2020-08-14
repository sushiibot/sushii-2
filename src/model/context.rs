use serenity::prelude::TypeMapKey;
use std::sync::Arc;

use super::{sushii_cache::SushiiCache, sushii_config::SushiiConfig};

#[derive(Clone)]
pub struct SushiiContext {
    pub config: Arc<SushiiConfig>,
    pub sushii_cache: SushiiCache,
    pub pool: sqlx::PgPool,
}

impl TypeMapKey for SushiiContext {
    type Value = SushiiContext;
}
