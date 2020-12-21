use serenity::prelude::TypeMapKey;
use sqlx::PgPool;

pub use crate::model::SushiiCache;

impl TypeMapKey for SushiiCache {
    type Value = SushiiCache;
}

pub struct DbPool;

impl TypeMapKey for DbPool {
    type Value = PgPool;
}
