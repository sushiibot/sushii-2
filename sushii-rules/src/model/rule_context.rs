use aho_corasick::AhoCorasick;
use anyhow::Result;
use dashmap::DashMap;
use handlebars::Handlebars;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::Arc;
use tokio::sync::RwLock;
use twilight_http::client::Client;
use twilight_model::id::GuildId;

type WordList = Arc<DashMap<GuildId, DashMap<String, AhoCorasick>>>;

/// This is shared to be accessed in the rules parsing.
/// Created each time an event fires.
#[derive(Debug, Clone)]
pub struct RuleContext<'a> {
    pub http: Client,
    pub reqwest: reqwest::Client,
    pub language_client: language_api_wrapper::LanguageApiClient,
    pub handlebars_templates: Arc<RwLock<Handlebars<'a>>>,
    pub word_lists: WordList,
    pub data: DashMap<String, serde_json::Value>,
}

impl<'a> RuleContext<'a> {
    pub fn new(
        http: Client,
        reqwest: reqwest::Client,
        language_client: language_api_wrapper::LanguageApiClient,
        handlebars_templates: Arc<RwLock<Handlebars<'a>>>,
        word_lists: WordList,
    ) -> Self {
        Self {
            http,
            reqwest,
            language_client,
            handlebars_templates,
            word_lists,
            data: DashMap::new(),
        }
    }

    pub async fn render_string(&self, input: &str) -> Result<String> {
        let mut hasher = DefaultHasher::new();
        hasher.write(input.as_bytes());
        let hash = format!("{:x}", hasher.finish());

        if !self.handlebars_templates.read().await.has_template(&hash) {
            self.handlebars_templates
                .write()
                .await
                .register_template_string(&hash, input)?;
        }

        self.handlebars_templates
            .read()
            .await
            .render(&hash, &self.data)
            .map_err(Into::into)
    }
}
