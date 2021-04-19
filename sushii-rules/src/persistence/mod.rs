use crate::error::Result;
use crate::model::{Rule, Trigger};

pub mod postgres;

pub trait RuleStore {
    /// Fetches matched rules based on trigger
    fn get_rule_from_trigger(guild_id: u64, trigger: Trigger) -> Result<Vec<Rule>>;

    /// Caches a rule
    fn cache_rule(&self, guild_id: u64, rule: Rule) -> Result<bool>;
}
