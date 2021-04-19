use aho_corasick::AhoCorasick;
use dashmap::DashMap;

/// This is shared to be accessed in the rules parsing.
/// Created each time an event fires.
#[derive(Debug, Clone)]
pub struct Context {
    /// Guild specific word lists
    pub word_lists: DashMap<String, AhoCorasick>,
}
