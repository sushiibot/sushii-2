pub struct WordLists {
    // Constructed word lists
    cache: Arc<RwLock<HashMap<GuildId, GuildWordList>>>,
    // Global word lists, common ones that servers may use as to not have to 
    // create and maintain their own list. This can be a list of things like
    // phishing links or common swear words.
    global_cache: Arc<RwLock<HashMap<String, AhoCorasick>>>
}

impl WordLists {
    pub fn guild_word_list(&self, guild_id: GuildId) -> GuildWorldLists {
        GuildWorldLists {
            cache: self.cache.read().await.get(&guild_id).cloned(),
            global_cache: self.global_cache.clone(),
        }
    }
}

pub struct GuildWorldLists {
    cache: Option<Arc<RwLock<HashMap<String, AhoCorasick>>>>,
    global_cache: Arc<RwLock<HashMap<String, AhoCorasick>>>
}
