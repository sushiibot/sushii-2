use anyhow::Result;
use reqwest::{Client, ClientBuilder};
use sqlx::PgPool;
use twilight_http::Client as TwilightClient;

#[derive(Clone, Debug)]
pub struct Context {
    pub http: TwilightClient,
    pub db_pool: PgPool,
    pub client: Client,
}

impl Context {
    pub fn new(http: TwilightClient, db_pool: PgPool) -> Result<Self> {
        let ctx = Self {
            http,
            db_pool,
            client: ClientBuilder::new()
                .user_agent(concat!(
                    env!("CARGO_PKG_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION")
                ))
                .build()?,
        };

        Ok(ctx)
    }
}
