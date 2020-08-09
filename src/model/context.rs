use std::sync::Arc;

use twilight::{
    cache::InMemoryCache, gateway::Cluster, http::Client as HttpClient,
};

use crate::model::{sushii_config::SushiiConfig, commands::Commands};

#[derive(Clone)]
pub struct SushiiContext<'a> {
    pub config: Arc<SushiiConfig>,
    pub cache: InMemoryCache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub pool: sqlx::PgPool,
    // pub command_parser: Parser<'a>,
    pub commands: Commands<'a>,
}
