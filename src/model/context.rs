use serenity::prelude::TypeMapKey;
use std::sync::Arc;

use super::{SushiiCache, SushiiConfig};

#[derive(Clone)]
pub struct SushiiContext {
    pub config: Arc<SushiiConfig>,
    pub sushii_cache: SushiiCache,
    pub pool: sqlx::PgPool,
}

impl TypeMapKey for SushiiContext {
    type Value = SushiiContext;
}
