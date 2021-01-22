use anyhow::Result;
use reqwest::{Client, ClientBuilder};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct Context {
    pub db_pool: PgPool,
    pub client: Client,
}

impl Context {
    pub fn new(db_pool: PgPool) -> Result<Self> {
        let ctx = Self {
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
