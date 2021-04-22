use aho_corasick::AhoCorasick;
use dashmap::DashMap;
use std::sync::Arc;
use twilight_http::client::Client;
use twilight_model::id::GuildId;

type WordList = Arc<DashMap<GuildId, DashMap<String, AhoCorasick>>>;

/// This is shared to be accessed in the rules parsing.
/// Created each time an event fires.
#[derive(Debug, Clone)]
pub struct Context {
    pub http: Client,
    pub word_lists: WordList,
    pub data: DashMap<String, serde_json::Value>,
}

impl Context {
    pub fn new(http: Client, word_lists: WordList) -> Self {
        Self {
            http,
            word_lists,
            data: DashMap::new(),
        }
    }
}
