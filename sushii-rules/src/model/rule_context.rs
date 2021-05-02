use aho_corasick::AhoCorasick;
use anyhow::Result;
use dashmap::DashMap;
use handlebars::Handlebars;
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use twilight_http::client::Client;
use twilight_model::id::GuildId;

use sushii_model::model::sql::GuildConfig;

use crate::model::Event;

#[derive(Debug, Default, Clone, Serialize)]
pub struct RuleContextData {
    /// Data from event that triggered this rule
    pub trigger: Option<Arc<Event>>,
    pub conditions: Option<serde_json::Value>,
    /// Data from each action, in order of execution
    pub actions: Vec<serde_json::Value>,
}

type WordList = Arc<DashMap<GuildId, DashMap<String, AhoCorasick>>>;

/// This is shared to be accessed in the rules parsing.
/// Created each time an event fires.
#[derive(Debug, Clone)]
pub struct RuleContext<'a> {
    pub guild_config: Arc<GuildConfig>,
    pub http: Client,
    pub pg_pool: sqlx::PgPool,
    pub reqwest: reqwest::Client,
    pub language_client: language_api_wrapper::LanguageApiClient,
    pub handlebars_templates: Arc<RwLock<Handlebars<'a>>>,
    pub word_lists: WordList,
    pub data: RuleContextData,
    pub channel_tx: Sender<Event>,
}

impl<'a> RuleContext<'a> {
    pub fn new(
        guild_config: Arc<GuildConfig>,
        http: Client,
        pg_pool: sqlx::PgPool,
        reqwest: reqwest::Client,
        language_client: language_api_wrapper::LanguageApiClient,
        handlebars_templates: Arc<RwLock<Handlebars<'a>>>,
        word_lists: WordList,
        channel_tx: Sender<Event>,
    ) -> Self {
        Self {
            guild_config,
            http,
            pg_pool,
            reqwest,
            language_client,
            handlebars_templates,
            word_lists,
            data: RuleContextData::default(),
            channel_tx,
        }
    }

    pub async fn render_string(&mut self, event: Arc<Event>, input: &str) -> Result<String> {
        // Hash template string so that the same template used in multiple
        // places will use the same pre-compiled template
        let mut hasher = DefaultHasher::new();
        hasher.write(input.as_bytes());
        let hash = format!("{:x}", hasher.finish());

        // Insert context data when needed instead of on load as not all rules
        // will use this
        if self.data.trigger.is_none() {
            tracing::debug!("Inserting trigger to handlebars context data");

            self.data.trigger = Some(event);
        }

        if !self.handlebars_templates.read().await.has_template(&hash) {
            self.handlebars_templates
                .write()
                .await
                .register_template_string(&hash, input)?;
        }

        tracing::debug!("Rendering template with context: {:?}", self.data);

        self.handlebars_templates
            .read()
            .await
            .render(&hash, &self.data)
            .map_err(Into::into)
    }
}
