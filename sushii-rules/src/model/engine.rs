use aho_corasick::AhoCorasick;
use anyhow::Result;
use dashmap::DashMap;
use handlebars::Handlebars;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use twilight_http::client::Client;
use twilight_model::id::GuildId;

use sushii_model::model::sql::GuildConfig;

use crate::model::has_id::HasGuildId;
use crate::model::{Event, Rule, RuleContext, Trigger};
use crate::persistence::RuleStore;

type RuleList = Vec<Arc<Rule>>;
type GuildTriggerRules = DashMap<Trigger, RuleList>;
type GuildRulesMap = DashMap<GuildId, GuildTriggerRules>;

type GuildConfigMap = DashMap<GuildId, GuildConfig>;

#[derive(Debug)]
pub struct RulesEngine {
    /// Stores rules fetched from file or database
    pub guild_rules: Arc<GuildRulesMap>,
    /// Rules persistence backend, use this to fetch rules
    pub rules_store: Box<dyn RuleStore>,
    /// Stores rules fetched from file or database
    pub guild_configs: Arc<GuildConfigMap>,
    /// Shared handlebars template to prevent reparsing
    /// This is a RwLock since registering templates requires mut self
    pub handlebars_templates: Arc<RwLock<Handlebars<'static>>>,
    /// Postgres database pool
    pub pg_pool: sqlx::PgPool,
    /// Guild specific word lists
    pub word_lists: Arc<DashMap<GuildId, DashMap<String, AhoCorasick>>>,
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
        rules_store: Box<dyn RuleStore>,
        language_api_endpoint: &str,
        channel_tx: Sender<Event>,
    ) -> Self {
        let reqwest = reqwest::Client::new();

        Self {
            guild_rules: Arc::new(DashMap::new()),
            rules_store,
            guild_configs: Arc::new(DashMap::new()),
            handlebars_templates: Arc::new(RwLock::new(Handlebars::new())),
            pg_pool,
            word_lists: Arc::new(DashMap::new()),
            http,
            reqwest: reqwest.clone(),
            language_client: language_api_wrapper::LanguageApiClient::new(
                reqwest,
                language_api_endpoint,
            ),
            channel_tx,
        }
    }

    /// Fetches rules from cache or from persistent store if not cached
    #[tracing::instrument]
    fn get_guild_rules(&self, guild_id: GuildId) -> Result<GuildTriggerRules> {
        if let Some(rules) = self.guild_rules.get(&guild_id) {
            Ok(rules.clone())
        } else {
            let guild_rules = self.rules_store.get_guild_rules(guild_id.0)?;

            let map = DashMap::new();
            for rule in guild_rules {
                let mut entry = map.entry(rule.trigger).or_insert_with(Vec::new);
                entry.push(Arc::new(rule));
            }

            self.guild_rules.insert(guild_id, map);
            Ok(self.guild_rules.get(&guild_id).unwrap().clone())
        }
    }

    /// Fetches the matching rules corresponding to a given event or trigger
    fn get_matching_rules(&self, event: &Arc<Event>) -> Result<Option<RuleList>> {
        let guild_id = match event.guild_id() {
            Ok(id) => id,
            Err(_) => {
                return Ok(None);
            }
        };

        let guild_rules = self.get_guild_rules(guild_id)?;

        // This will be Counter or a Twilight(EventType)
        let event_type = event.kind();
        Ok(guild_rules.get(&event_type.into()).map(|r| r.clone()))
    }

    /*
    pub async fn trigger_stream(&mut self) {
        let channel_rx = self.channel_rx.take().expect("Receiving channel already taken!");

        tokio::spawn(async move {
            while let Some(event) = channel_rx.recv().await {
                if let Err(e) = self.process_event(Arc::new(event)) {
                    tracing::error!("Failed to process triggered event: {}", e);
                }
            }
        });
    }
    */

    /// Events that modify counters also trigger this to process the counter.
    /// It provides the original event that triggered this counter
    #[tracing::instrument]
    pub fn process_event(&self, event: Arc<Event>) -> Result<()> {
        let matching_rules = match self.get_matching_rules(&event)? {
            Some(r) => r,
            None => return Ok(()),
        };

        for rule in matching_rules.iter() {
            // Create a new context on every rule trigger
            let context = RuleContext::new(
                self.http.clone(),
                self.pg_pool.clone(),
                self.reqwest.clone(),
                self.language_client.clone(),
                self.handlebars_templates.clone(),
                self.word_lists.clone(),
                self.channel_tx.clone(),
            );

            let event = event.clone();
            let rule = rule.clone();

            tokio::spawn(async move {
                let start = Instant::now();

                if let Err(e) = rule.check_event(event, &context).await {
                    tracing::warn!("Failed checking event: {}", e);
                }

                let delta = start.elapsed();
                metrics::histogram!("rule_execution", delta);
            });
        }

        Ok(())
    }
}
