use anyhow::Result;
use dashmap::DashMap;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tonic::{transport::Server, Request, Response, Status};
use tracing_subscriber::filter::EnvFilter;

mod model;
mod update;
use model::context::Context;

use sushii_feeds::feed_request::feed_service_server::{FeedService, FeedServiceServer};
use sushii_feeds::feed_request::{feed_update_reply::FeedItem, Empty, FeedUpdateReply};

#[derive(Debug)]
pub struct GrpcService {
    ctx: Context,
    cache: DashMap<String, (Instant, Vec<FeedItem>)>,
    /// Default cache ttl is 1 minute
    ttl: Duration,
}

impl GrpcService {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
            cache: DashMap::new(),
            ttl: Duration::from_secs(60),
        }
    }

    pub fn update(&self, key: impl Into<String>, items: Vec<FeedItem>) {
        self.cache.insert(key.into(), (Instant::now(), items));
    }

    pub fn get(&self, key: &str) -> Option<Vec<FeedItem>> {
        let entry = self.cache.get(key)?;
        let (last_update, items) = entry.value();

        let now = Instant::now();
        let age = now.duration_since(*last_update);

        // Older than ttl, return None to get new items
        if age > self.ttl {
            tracing::debug!("Cache item stale");
            return None;
        }

        // Younger than ttl, cache still fresh so return cache content
        Some(items.clone())
    }
}

#[tonic::async_trait]
impl FeedService for GrpcService {
    async fn update_feeds(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<FeedUpdateReply>, Status> {
        tracing::info!("Got a request from {:?}", request.remote_addr());

        // Check cache
        let items = if let Some(items) = self.get("vlive") {
            items
        } else {
            let items = update::update_vlive(self.ctx.clone())
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            // Update cache
            self.update("vlive", items.clone());

            items
        };

        let reply = FeedUpdateReply { items };

        Ok(Response::new(reply))
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

    let listen_addr: SocketAddr = env::var("GRPC_ADDR")
        .map(|s| {
            s.parse::<SocketAddr>()
                .expect("Failed to parse GRPC_ADDR as a SocketAddr")
        })
        .unwrap_or_else(|_| "0.0.0.0:50051".parse().unwrap());

    let ctx = Context::new(db_pool)?;

    tracing::info!("Feed gRPC server listening on {}", listen_addr);

    let service = GrpcService::new(ctx);

    Server::builder()
        .add_service(FeedServiceServer::new(service))
        .serve(listen_addr)
        .await?;

    // run(ctx).await;

    Ok(())
}
