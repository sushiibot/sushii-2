use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use tokio::{
    task,
    time::{self, Duration},
};
use tonic::{transport::Server, Request, Response, Status};
use tracing_subscriber::filter::EnvFilter;
use vlive::VLiveRequester;

mod model;
mod update;
use model::context::Context;

use sushii_feeds::feed_request::feed_service_server::{FeedService, FeedServiceServer};
use sushii_feeds::feed_request::{feed_update_reply::FeedItem, Empty, FeedUpdateReply};

#[derive(Debug)]
pub struct GrpcService {
    ctx: Context,
    last_update: Option<NaiveDateTime>,
}

impl GrpcService {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
}

#[tonic::async_trait]
impl FeedService for GrpcService {
    async fn update_feeds(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<FeedUpdateReply>, Status> {
        tracing::info!("Got a request from {:?}", request.remote_addr());

        let reply = FeedUpdateReply {
            items: update::update_vlive(self.ctx.clone())
                .await
                .map_err(|e| Status::internal(e.to_string()))?,
        };

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
