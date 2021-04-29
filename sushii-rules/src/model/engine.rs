use aho_corasick::AhoCorasick;
use anyhow::Result;
use dashmap::DashMap;
use handlebars::Handlebars;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use twilight_http::client::Client;
use twilight_model::id::GuildId;

use crate::model::{Rule, Event, RuleContext, Trigger};
use crate::model::has_id::HasGuildId;
use crate::persistence::RuleStore;

type RuleList = Vec<Arc<Rule>>;
type GuildTriggerRules = DashMap<Trigger, RuleList>;
type GuildRulesMap = DashMap<GuildId, GuildTriggerRules>;

#[derive(Clone, Debug)]
pub struct RulesEngine {
    /// Stores rules fetched from file or database
    pub guild_rules: Arc<GuildRulesMap>,
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
    fn get_matching_rules(
        &self,
        event: &Arc<Event>,
        trigger_override: Option<Trigger>,
    ) -> Result<Option<RuleList>> {
        let guild_id = match event.guild_id() {
            Ok(id) => id,
            Err(_) => {
                return Ok(None);
            }
        };

        let guild_rules = self.get_guild_rules(guild_id)?;

        if let Some(trigger) = trigger_override {
            Ok(guild_rules.get(&trigger).map(|r| r.clone()))
        } else {
            let event_type = event.kind();

            Ok(guild_rules.get(&event_type.into()).map(|r| r.clone()))
        }
    }

    /// Events that modify counters trigger this to process the counter separately
    /// It provides the original event that triggered this counter
    pub fn process_counter(&self, event: Arc<Event>) -> Result<()> {
        let matching_rules = match self.get_matching_rules(&event, Some(Trigger::Counter))? {
            Some(r) => r,
            None => return Ok(()),
        };

        Ok(())
    }

    #[tracing::instrument]
    pub fn process_event(&self, event: Event) -> Result<()> {
        let event = Arc::new(event);
        let matching_rules = match self.get_matching_rules(&event, None)? {
            Some(r) => r,
            None => return Ok(()),
        };

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
