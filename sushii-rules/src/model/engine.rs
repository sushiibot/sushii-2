use aho_corasick::AhoCorasick;
use anyhow::Result;
use dashmap::DashMap;
use handlebars::Handlebars;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use twilight_http::client::Client;
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::id::GuildId;

use super::{Rule, RuleContext, Trigger};
use crate::model::has_id::HasGuildId;
use crate::persistence::RuleStore;

#[derive(Clone, Debug)]
pub struct RulesEngine {
    /// Stores rules fetched from file or database
    pub guild_rules: Arc<DashMap<GuildId, DashMap<Trigger, Vec<Arc<Rule>>>>>,
    /// Rules persistence backend, use this to fetch rules
    pub rules_store: Box<dyn RuleStore>,
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
}

impl RulesEngine {
    pub fn new(
        http: Client,
        pg_pool: sqlx::PgPool,
        rules_store: Box<dyn RuleStore>,
        language_api_endpoint: &str,
    ) -> Self {
        let reqwest = reqwest::Client::new();

        Self {
            guild_rules: Arc::new(DashMap::new()),
            rules_store,
            handlebars_templates: Arc::new(RwLock::new(Handlebars::new())),
            pg_pool,
            word_lists: Arc::new(DashMap::new()),
            http,
            reqwest: reqwest.clone(),
            language_client: language_api_wrapper::LanguageApiClient::new(
                reqwest,
                language_api_endpoint,
            ),
        }
    }

    #[tracing::instrument]
    pub fn process_event(&self, event: DispatchEvent) -> Result<()> {
        let guild_id = match event.guild_id() {
            Some(id) => id,
            None => {
                return Ok(());
            }
        };

        let guild_rules = match self.guild_rules.get(&guild_id) {
            Some(r) => r,
            None => {
                let guild_rules = self.rules_store.get_guild_rules(guild_id.0)?;

                let map = DashMap::new();
                for rule in guild_rules {
                    let mut entry = map.entry(rule.trigger).or_insert_with(Vec::new);
                    entry.push(Arc::new(rule));
                }

                self.guild_rules.insert(guild_id, map);
                self.guild_rules.get(&guild_id).unwrap()
            }
        };

        let event_type = event.kind();
        let matching_rules = match guild_rules.get(&event_type.into()) {
            Some(r) => r,
            None => {
                return Ok(());
            }
        };

        let event = Arc::new(event);

        for rule in matching_rules.iter() {
            let context = RuleContext::new(
                self.http.clone(),
                self.pg_pool.clone(),
                self.reqwest.clone(),
                self.language_client.clone(),
                self.handlebars_templates.clone(),
                self.word_lists.clone(),
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
