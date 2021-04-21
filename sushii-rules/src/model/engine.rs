use aho_corasick::AhoCorasick;
use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use twilight_http::client::Client;
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::gateway::payload;
use twilight_model::id::GuildId;

use super::{Context, Rule, Trigger};

#[derive(Clone, Debug)]
pub struct RulesEngine {
    /// Stores rules fetched from file or database
    pub rules_cache: Arc<DashMap<GuildId, DashMap<Trigger, Vec<Arc<Rule>>>>>,
    /// Guild specific word lists
    pub word_lists: Arc<DashMap<GuildId, DashMap<String, AhoCorasick>>>,
    pub http: Client,
}

impl RulesEngine {
    pub fn new(http: Client) -> Self {
        Self {
            rules_cache: Arc::new(DashMap::new()),
            word_lists: Arc::new(DashMap::new()),
            http,
        }
    }

    #[tracing::instrument]
    pub fn process_event(&self, event: DispatchEvent) -> Result<()> {
        let guild_id = match event.guild_id() {
            Some(id) => id,
            None => {
                tracing::debug!("No guild_id found for event, ignoring");

                return Ok(());
            }
        };

        let guild_rules = match self.rules_cache.get(&guild_id) {
            Some(r) => r,
            None => {
                tracing::debug!(?guild_id, "No rules found for guild");
                return Ok(());
            }
        };

        let event_type = event.kind();
        let matching_rules = match guild_rules.get(&event_type.into()) {
            Some(r) => r,
            None => {
                tracing::debug!(
                    ?guild_id,
                    ?event_type,
                    "No rules with matching trigger found for guild"
                );
                return Ok(());
            }
        };

        let event = Arc::new(event);

        for rule in matching_rules.iter() {
            let context = Context::new();
            let event = event.clone();
            let rule = rule.clone();

            tokio::spawn(async move {
                rule.check_event(event, &context).await;
            });
        }

        Ok(())
    }
}

pub trait HasGuildId {
    fn guild_id(&self) -> Option<GuildId>;
}

impl HasGuildId for DispatchEvent {
    fn guild_id(&self) -> Option<GuildId> {
        match *self {
            Self::BanAdd(payload::BanAdd { guild_id, .. }) => Some(guild_id),
            Self::BanRemove(payload::BanRemove { guild_id, .. }) => Some(guild_id),
            Self::MessageCreate(ref msg) => msg.guild_id,
            Self::MemberAdd(ref member) => Some(member.guild_id),
            _ => None,
        }
    }
}
