use serenity::prelude::*;
use std::env;
use std::net::IpAddr;
use std::sync::Arc;

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct SushiiConfig {
    pub discord_token: String,
    pub owner_ids: Vec<u64>,
    pub database_url: String,
    pub default_prefix: String,
    pub blocked_users: Vec<u64>,
    pub lastfm_key: String,
    pub metrics_interface: IpAddr,
    pub metrics_port: u16,
    pub sentry_dsn: Option<String>,
    pub image_server_url: String,
}

fn parse_id_array(s: &str) -> Vec<u64> {
    s.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|u| u.trim().parse::<u64>().ok())
        .collect()
}

impl SushiiConfig {
    pub fn new_from_env() -> Result<Self> {
        if let Err(e) = dotenv::dotenv() {
            tracing::warn!(
                "Failed to read .env file ({}), checking if environment variables already exist",
                e
            );
        }

        Ok(SushiiConfig {
            discord_token: env::var("DISCORD_TOKEN")?,
            owner_ids: parse_id_array(&env::var("OWNER_IDS").unwrap_or_else(|_| "".into())),
            database_url: env::var("DATABASE_URL")?,
            default_prefix: env::var("DEFAULT_PREFIX")?,
            blocked_users: parse_id_array(&env::var("BLOCKED_USERS").unwrap_or_else(|_| "".into())),
            lastfm_key: env::var("LASTFM_KEY").unwrap_or_else(|_| "".into()),
            // Default expose on 0.0.0.0 to let other Docker containers access
            metrics_interface: env::var("METRICS_INTERFACE")
                .ok()
                .and_then(|x| x.parse().ok())
                .unwrap_or_else(|| "0.0.0.0".parse().unwrap()),
            metrics_port: env::var("METRICS_PORT")
                .ok()
                .and_then(|x| x.parse().ok())
                .unwrap_or(9888),
            sentry_dsn: env::var("SENTRY_DSN").ok(),
            image_server_url: env::var("IMAGE_SERVER_URL")?,
        })
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

#[test]
fn parses_array() {
    let expected = vec![123, 456, 789];
    assert_eq!(parse_id_array("123,456,789"), expected);
    assert_eq!(parse_id_array("123, 456, 789"), expected);
    assert_eq!(parse_id_array("123, 456   , 789         "), expected);
}
