use aho_corasick::AhoCorasick;
use dashmap::DashMap;
use std::sync::Arc;
use twilight_http::client::Client;
use twilight_model::id::GuildId;

type WordList = Arc<DashMap<GuildId, DashMap<String, AhoCorasick>>>;

/// This is shared to be accessed in the rules parsing.
/// Created each time an event fires.
#[derive(Debug, Clone)]
pub struct RuleContext {
    pub http: Client,
    pub reqwest: reqwest::Client,
    pub language_client: language_api_wrapper::LanguageApiClient,
    pub word_lists: WordList,
    pub data: DashMap<String, serde_json::Value>,
}

impl RuleContext {
    pub fn new(
        http: Client,
        reqwest: reqwest::Client,
        language_client: language_api_wrapper::LanguageApiClient,
        word_lists: WordList,
    ) -> Self {
        Self {
            http,
            reqwest,
            language_client,
            word_lists,
            data: DashMap::new(),
        }
    }
}
