use std::sync::Arc;
use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};
use sqlx::PgPool;

pub use crate::model::{
    sushii_cache::SushiiCache,
    sushii_config::SushiiConfig
};

impl TypeMapKey for SushiiCache {
    type Value = SushiiCache;
}

impl TypeMapKey for SushiiConfig {
    type Value = SushiiConfig;
}

pub struct DbPool;

impl TypeMapKey for DbPool {
    type Value = PgPool;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}