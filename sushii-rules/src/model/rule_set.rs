use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use twilight_model::gateway::event::DispatchEvent;

use crate::model::{Action, Condition, Rule, RuleContext, Trigger};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RuleSet {
    pub id: Uuid,
    /// Guild ID this rule set belongs to
    pub guild_id: u64,
    /// Name of this rule set, should be the feature name
    pub name: String,
    /// Description of rule set
    pub description: Option<String>,
    /// If this rule set is enabled or not, should pass down to all containing
    /// rules
    pub enabled: bool,
    /// If the guild can edit this rule set
    pub editable: bool,
    /// Author ID of this rule set, for display in web UI
    pub author: u64,
    /// Rule set category, e.g. moderation, fun, etc.
    pub category: Option<String>,
    /// Rule set configuration, map of json values
    pub config: HashMap<String, Value>,
    /// List of rules in this rule set
    pub rules: Vec<Rule>,
}
