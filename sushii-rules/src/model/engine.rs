use aho_corasick::AhoCorasick;
use handlebars::Handlebars;
use std::collections::HashMap;

use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use twilight_http::client::Client;
use twilight_model::id::GuildId;

use crate::error::{Error, Result};
use crate::model::has_id::HasGuildId;
use crate::model::{
    cache::{GuildConfigCache, RuleSetsCache},
    Event, RuleContext, RuleSet,
};
use crate::persistence::RuleStore;

pub struct RulesEngine {
    /// Stores rules fetched from file or database
    pub guild_configs: GuildConfigCache,
    /// Shared handlebars template to prevent reparsing
    /// This is a RwLock since registering templates requires mut self
    pub handlebars_templates: Arc<RwLock<Handlebars<'static>>>,
    /// Postgres database pool
    pub pg_pool: sqlx::PgPool,
    /// Redis connection pool
    pub redis_pool: deadpool_redis::Pool,
    /// Guild specific word lists
    pub word_lists: Arc<RwLock<HashMap<GuildId, Arc<RwLock<HashMap<String, AhoCorasick>>>>>>,
    /// Twilight HTTP client
    pub http: Client,
    pub reqwest: reqwest::Client,
    /// Wraps the reqwest client
    pub language_client: language_api_wrapper::LanguageApiClient,
    /// Counter triggers from other events
    /// Events can send a new counter event
    pub channel_tx: Sender<Event>,
}

impl RulesEngine {
    pub fn new(
        http: Client,
        pg_pool: sqlx::PgPool,
        redis_pool: deadpool_redis::Pool,
        language_api_endpoint: &str,
        channel_tx: Sender<Event>,
    ) -> Self {
        let reqwest = reqwest::Client::new();

        Self {
            guild_configs: GuildConfigCache::new(),
            handlebars_templates: Arc::new(RwLock::new(Handlebars::new())),
            pg_pool,
            redis_pool,
            word_lists: Arc::new(RwLock::new(HashMap::new())),
            http,
            reqwest: reqwest.clone(),
            language_client: language_api_wrapper::LanguageApiClient::new(
                reqwest,
                language_api_endpoint,
            ),
            channel_tx,
        }
    }

    /// Events that modify counters also trigger this to process the counter.
    /// It provides the original event that triggered this counter
    #[tracing::instrument(skip(self))]
    pub async fn process_event(&self, event: Arc<Event>) -> Result<()> {
        let event_type = match event.kind() {
            Ok(t) => t,
            Err(Error::UnsupportedEvent) => return Ok(()),
            Err(e) => return Err(e),
        };

        // Only process guild events
        let guild_id = match event.guild_id() {
            Ok(id) => id,
            Err(_) => return Ok(()),
        };

        // Fetch guild rule sets
        let guild_rule_sets =
            RuleSet::sets_from_guild_id(self.redis_pool.clone(), &self.pg_pool, guild_id.0).await?;

        if guild_rule_sets.is_empty() {
            return Ok(());
        }

        let guild_config = self.guild_configs.get(&self.pg_pool, guild_id).await?;

        for rule_set in guild_rule_sets {
            if !rule_set.enabled {
                continue;
            }

            for rule in rule_set.rules.iter() {
                if !rule.enabled {
                    continue;
                }

                if rule.trigger != event_type {
                    continue;
                }

                // Create a new context on every rule trigger
                let mut context = RuleContext::new(
                    guild_config.clone(),
                    self.http.clone(),
                    self.pg_pool.clone(),
                    self.reqwest.clone(),
                    self.language_client.clone(),
                    self.handlebars_templates.clone(),
                    self.word_lists.read().await.get(&guild_id).cloned(),
                    self.channel_tx.clone(),
                );

                let event = event.clone();
                let rule = rule.clone();

                tokio::spawn(async move {
                    let start = Instant::now();

                    if let Err(e) = rule.check_event(event, &mut context).await {
                        tracing::warn!("Failed checking event: {}", e);
                    }

                    let delta = start.elapsed();
                    metrics::histogram!("rule_execution", delta);
                });
            }
        }

        Ok(())
    }
}
