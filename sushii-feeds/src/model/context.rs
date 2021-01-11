use anyhow::Result;
use darkredis::ConnectionPool;
use reqwest::{Client, ClientBuilder};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct Context {
    db_pool: PgPool,
    redis_pool: ConnectionPool,
    client: Client,
}

impl Context {
    pub fn new(db_pool: PgPool, redis_pool: ConnectionPool) -> Result<Self> {
        let ctx = Self {
            db_pool,
            redis_pool,
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
