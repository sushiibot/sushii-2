use anyhow::Result;
use reqwest::{Client, ClientBuilder};
use sqlx::PgPool;
use std::time::Duration;
use twilight_http::Client as TwilightClient;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Context {
    pub http: Arc<TwilightClient>,
    pub db_pool: PgPool,
    pub client: Client,
}

impl Context {
    pub fn new(http: Arc<TwilightClient>, db_pool: PgPool) -> Result<Self> {
        let ctx = Self {
            http,
            db_pool,
            client: ClientBuilder::new()
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION")
                ))
                .timeout(Duration::from_secs(30))
                .build()?,
        };

        Ok(ctx)
    }
}
