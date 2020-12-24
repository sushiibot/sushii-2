use reqwest::Client as ReqwestClient;
use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::{Mutex, TypeMapKey},
    CacheAndHttp,
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

pub struct CacheAndHttpContainer;

impl TypeMapKey for CacheAndHttpContainer {
    type Value = Arc<CacheAndHttp>;
}

pub struct ReqwestContainer;

impl TypeMapKey for ReqwestContainer {
    type Value = ReqwestClient;
}
