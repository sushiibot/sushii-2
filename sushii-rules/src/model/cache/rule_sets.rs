use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use twilight_model::id::GuildId;

use crate::model::has_id::HasGuildId;
use crate::model::{Event, Rule, RuleSet, Trigger};
use crate::persistence::RuleStore;

/// A single rule set with a rule cache
#[derive(Debug)]
pub struct RuleSetCacheItem {
    rule_set: RuleSet,
    /// Trigger -> Vec<Rule> for rules only in this rule set
    trigger_map: DashMap<Trigger, Vec<Arc<Rule>>>,
}

type GuildRuleSet = Vec<Arc<RuleSetCacheItem>>;
type GuildRuleSets = DashMap<GuildId, GuildRuleSet>;

#[derive(Debug)]
pub struct RuleSetsCache {
    guild_rule_sets: GuildRuleSets,
    /// Rules persistence backend, use this to fetch rules
    rules_store: Box<dyn RuleStore>,
}

impl RuleSetsCache {
    pub fn new(rules_store: Box<dyn RuleStore>) -> Self {
        Self {
            guild_rule_sets: DashMap::new(),
            rules_store,
        }
    }

    /// Fetches all of a guild's rule sets from cache or from persistent store
    /// if not cached
    #[tracing::instrument]
    fn get_guild_rule_sets(&self, guild_id: GuildId) -> Result<GuildRuleSet> {
        // If cached, return all
        if let Some(rule_sets) = self.guild_rule_sets.get(&guild_id) {
            Ok(rule_sets.clone())
        } else {
            let guild_rule_sets = self.rules_store.get_guild_rule_sets(guild_id.0)?;

            let mut sets = Vec::new();

            for rule_set in guild_rule_sets {
                // Trigger -> rule in a single rule set
                let trigger_map = DashMap::new();

                for rule in &rule_set.rules {
                    let mut entry = trigger_map.entry(rule.trigger).or_insert_with(Vec::new);
                    entry.push(Arc::new(rule.clone()));
                }

                sets.push(Arc::new(RuleSetCacheItem {
                    rule_set,
                    trigger_map,
                }));
            }

            self.guild_rule_sets.insert(guild_id, sets);
            Ok(self.guild_rule_sets.get(&guild_id).unwrap().clone())
        }
    }

    /// Fetches the matching rules corresponding to a given event or trigger.
    /// These rules can come from any enabled rule set
    pub fn get_matching_rules(&self, event: &Arc<Event>) -> Result<Vec<Arc<Rule>>> {
        let guild_id = match event.guild_id() {
            Ok(id) => id,
            Err(_) => {
                return Ok(Vec::new());
            }
        };

        let guild_rule_sets = self.get_guild_rule_sets(guild_id)?;

        let mut matching_rules = Vec::new();

        // This will be Counter or a Twilight(EventType)
        let event_type = event.kind();

        for set in guild_rule_sets {
            // Skip all rules in rule set if disabled
            if !set.rule_set.enabled {
                continue;
            }

            if let Some(rules) = set.trigger_map.get(&event_type).map(|r| r.clone()) {
                for rule in rules {
                    matching_rules.push(rule.clone());
                }
            }
        }

        Ok(matching_rules)
    }
}
