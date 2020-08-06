use std::sync::Arc;

use twilight::{
    cache::InMemoryCache, command_parser::Parser, gateway::Cluster, http::Client as HttpClient,
};

use crate::model::{command::Command, sushii_config::SushiiConfig};

#[derive(Clone)]
pub struct SushiiContext<'a> {
    pub config: Arc<SushiiConfig>,
    pub cache: InMemoryCache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub pool: sqlx::PgPool,
    pub command_parser: Parser<'a>,
    // pub commands: Arc<dashmap::DashMap<String, Command>>,
}
