use aho_corasick::AhoCorasick;
use anyhow::Result;
use dashmap::DashMap;
use handlebars::Handlebars;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use twilight_http::client::Client;
use twilight_model::id::GuildId;

use crate::model::Event;

type WordList = Arc<DashMap<GuildId, DashMap<String, AhoCorasick>>>;

/// This is shared to be accessed in the rules parsing.
/// Created each time an event fires.
#[derive(Debug, Clone)]
pub struct RuleContext<'a> {
    pub http: Client,
    pub pg_pool: sqlx::PgPool,
    pub reqwest: reqwest::Client,
    pub language_client: language_api_wrapper::LanguageApiClient,
    pub handlebars_templates: Arc<RwLock<Handlebars<'a>>>,
    pub word_lists: WordList,
    pub data: DashMap<String, serde_json::Value>,
    pub channel_tx: Sender<Event>,
}

impl<'a> RuleContext<'a> {
    pub fn new(
        http: Client,
        pg_pool: sqlx::PgPool,
        reqwest: reqwest::Client,
        language_client: language_api_wrapper::LanguageApiClient,
        handlebars_templates: Arc<RwLock<Handlebars<'a>>>,
        word_lists: WordList,
        channel_tx: Sender<Event>,
    ) -> Self {
        Self {
            http,
            pg_pool,
            reqwest,
            language_client,
            handlebars_templates,
            word_lists,
            data: DashMap::new(),
            channel_tx,
        }
    }

    pub async fn render_string(&self, event: Arc<Event>, input: &str) -> Result<String> {
        // Hash template string so that the same template used in multiple
        // places will use the same pre-compiled template
        let mut hasher = DefaultHasher::new();
        hasher.write(input.as_bytes());
        let hash = format!("{:x}", hasher.finish());

        // Insert context data
        if !self.data.contains_key("trigger") {
            tracing::debug!("Inserting trigger to handlebars context data");

            self.data
                .insert("trigger".to_string(), serde_json::to_value(event)?);
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
