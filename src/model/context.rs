use std::sync::Arc;

use twilight::{
    cache::InMemoryCache, gateway::Cluster, http::Client as HttpClient,
};

use super::{sushii_config::SushiiConfig, commands::Commands, sushii_cache::SushiiCache};

#[derive(Clone)]
pub struct SushiiContext<'a> {
    pub config: Arc<SushiiConfig>,
    pub sushii_cache: SushiiCache,
    pub cache: InMemoryCache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub pool: sqlx::PgPool,
    // pub command_parser: Parser<'a>,
    pub commands: Commands<'a>,
}
