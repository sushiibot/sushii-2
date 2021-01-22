use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use sushii_model::model::sql::{Feed, FeedItem, FeedMetadata, FeedSubscription};
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

pub mod feed_request {
    tonic::include_proto!("feedrequest");
}

use feed_request::feed_service_server::{FeedService, FeedServiceServer};
use feed_request::{Empty, FeedUpdateReply};

#[derive(Debug)]
pub struct GrpcService {
    ctx: Context,
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
        println!("Got a request from {:?}", request.remote_addr());

        let reply = FeedUpdateReply { items: Vec::new() };
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
