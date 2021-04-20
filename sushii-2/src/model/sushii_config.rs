use serde::Deserialize;
use serenity::prelude::*;
use std::net::IpAddr;
use std::sync::Arc;

use crate::error::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct SushiiConfig {
    pub discord_token: String,
    pub database_url: String,
    pub default_prefix: String,
    pub lastfm_key: String,
    pub metrics_interface: IpAddr,
    pub metrics_port: u16,
    pub sentry_dsn: Option<String>,
    pub image_server_url: String,
    #[serde(default)]
    pub redis: deadpool_redis::Config,
}

impl SushiiConfig {
    pub fn from_env() -> Result<Self> {
        if let Err(e) = dotenv::dotenv() {
            tracing::warn!(
                "Failed to read .env file ({}), checking if environment variables already exist",
                e
            );
        }

        let mut cfg = config::Config::new();

        cfg.set_default("metrics_interface", "0.0.0.0")?
            .set_default("metrics_port", "9888")?;

        cfg.merge(config::Environment::new())?;

        cfg.try_into().map_err(Into::into)
    }

    pub async fn get(ctx: &Context) -> Arc<Self> {
        ctx.data
            .read()
            .await
            .get::<SushiiConfig>()
            .cloned()
            .expect("Context data is missing SushiiConfig")
    }
}
