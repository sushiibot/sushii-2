use deadpool_redis::Pool as RedisPool;
use reqwest::Client as ReqwestClient;
use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
};
use std::sync::Arc;

pub use crate::model::{Metrics, SushiiConfig};

impl TypeMapKey for SushiiConfig {
    type Value = Arc<SushiiConfig>;
}

impl TypeMapKey for Metrics {
    type Value = Arc<Metrics>;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct ReqwestContainer;

impl TypeMapKey for ReqwestContainer {
    type Value = ReqwestClient;
}

pub struct RedisPoolContainer;

impl TypeMapKey for RedisPoolContainer {
    type Value = RedisPool;
}
