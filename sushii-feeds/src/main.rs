use anyhow::Result;
use darkredis::ConnectionPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::{
    task,
    time::{self, Duration},
};
use tracing_subscriber::filter::EnvFilter;

mod model;
use model::context::Context;

async fn run(ctx: Context) {
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        // Wait 10 seconds
        interval.tick().await;

        // Spawn API fetching on task
        // task::spawn()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL in environment");

    let db_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let redis_addr = env::var("REDIS_HOST").expect("Missing REDIS_HOST in environment");
    let redis_pool = ConnectionPool::create(redis_addr, None, num_cpus::get()).await?;

    let ctx = Context::new(db_pool, redis_pool)?;

    run(ctx).await;

    Ok(())
}
