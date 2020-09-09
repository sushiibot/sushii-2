use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
    CacheAndHttp,
};
use sqlx::PgPool;
use std::sync::Arc;

pub use crate::model::{Metrics, MetricsAsync, SushiiCache, SushiiConfig, SushiiConfigDb};

impl TypeMapKey for SushiiCache {
    type Value = SushiiCache;
}

impl TypeMapKey for SushiiConfig {
    type Value = Arc<SushiiConfig>;
}

impl TypeMapKey for Metrics {
    type Value = Arc<Metrics>;
}

pub struct DbPool;

impl TypeMapKey for DbPool {
    type Value = PgPool;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct CacheAndHttpContainer;

impl TypeMapKey for CacheAndHttpContainer {
    type Value = Arc<CacheAndHttp>;
}
