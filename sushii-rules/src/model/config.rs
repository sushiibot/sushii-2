use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

use crate::model::{Action, RuleContext};
use crate::error::{Error, Result};

pub type RuleConfig = HashMap<String, ConfigValue>;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConfigValue {
    String(String),
    Strings(Vec<String>),
    Number(i64),
    Numbers(Vec<i64>),
    Bool(bool),
    Date(DateTime<Utc>),
    DiscordChannel(i64),
    DiscordChannels(Vec<i64>),
    DiscordRole(i64),
    DiscordRoles(Vec<i64>),
    Actions(Vec<Action>),
}

impl ConfigValue {
    pub fn as_str(&self) -> Option<Cow<'_, str>> {
        match self {
            Self::String(s) => Some(Cow::Borrowed(&s)),
            // Self::Strings(Vec<String>) => ,
            Self::Number(num) => Some(num.to_string().into()),
            _ => None,
        }
    }

    pub fn as_str_vec(&self) -> Option<Vec<Cow<'_, str>>> {
        match self {
            Self::Strings(s) => Some(s.iter().map(|s| Cow::Borrowed(s.as_str())).collect()),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Number(n) | Self::DiscordChannel(n) | Self::DiscordRole(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_i64_vec(&self) -> Option<&Vec<i64>> {
        match self {
            Self::Numbers(n) | Self::DiscordChannels(n) | Self::DiscordRoles(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_datetime(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Date(d) => Some(d.clone()),
            _ => None,
        }
    }

    pub fn as_actions(&self) -> Option<&Vec<Action>> {
        match self {
            Self::Actions(a) => Some(a),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConfigOrValue<T> {
    /// # Configuration Key
    /// Key to fetch from the rule configuration
    ConfigKey(String),
    /// # Value
    /// Single value to match directly
    Value(T),
}

pub trait ConfigGet<'a> {
    type Output;

    fn get(&'a self, ctx: &'a RuleContext<'_>) -> Result<Self::Output>;
}

impl<'a> ConfigGet<'a> for ConfigOrValue<String> {
    type Output = Cow<'a, str>;

    fn get(&'a self, ctx: &'a RuleContext<'_>) -> Result<Self::Output> {
        match self {
            Self::ConfigKey(key) => ctx
                .data
                .rule_config
                .get(key)
                .ok_or_else(|| Error::RuleConfigMissingField(key.clone().into()))?
                .as_str()
                .ok_or_else(|| {
                    Error::RuleConfigMismatchedType(key.clone().into(), "String".into())
                }),
            Self::Value(val) => Ok(Cow::Borrowed(val.as_str())),
        }
    }
}
