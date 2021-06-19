use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::model::{Action, RuleContext};

pub type RuleConfig = HashMap<String, serde_json::Value>;

// Types in constraints that can be either a user provided hardcoded value or
// a key for a typed config value. These are separate types as to be able to
// easily determine the data type required when scanning the JSON data. Using
// generics would reduce repeated code here, but would make it difficult to
// determine what each var type is in the client side.

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StringVar {
    /// # Value
    /// Value to match directly
    Value(String),
    /// # Configuration Key
    /// Key to fetch from the rule configuration
    ConfigKey(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StringVecVar {
    /// # Value
    /// Value to match directly
    Value(Vec<String>),
    /// # Configuration Key
    /// Key to fetch from the rule configuration
    ConfigKey(String),
}

pub trait ConfigGet<'a> {
    type Output;

    fn get(&'a self, ctx: &'a RuleContext<'_>) -> Result<Self::Output>;
}

impl<'a> ConfigGet<'a> for StringVar {
    type Output = Cow<'a, str>;

    fn get(&'a self, ctx: &'a RuleContext<'_>) -> Result<Self::Output> {
        match self {
            Self::ConfigKey(key) => ctx
                .data
                .rule_config
                .get(key)
                .ok_or_else(|| Error::RuleConfigMissingField(key.clone().into()))?
                .as_str()
                .map(|s| Cow::Borrowed(s))
                .ok_or_else(|| {
                    Error::RuleConfigMismatchedType(key.clone().into(), "String".into())
                }),
            Self::Value(val) => Ok(Cow::Borrowed(val.as_str())),
        }
    }
}

impl<'a> ConfigGet<'a> for StringVecVar {
    type Output = Vec<Cow<'a, str>>;

    fn get(&'a self, ctx: &'a RuleContext<'_>) -> Result<Self::Output> {
        match self {
            Self::ConfigKey(key) => ctx
                .data
                .rule_config
                .get(key)
                .ok_or_else(|| Error::RuleConfigMissingField(key.clone().into()))?
                .as_array()
                .ok_or_else(|| {
                    Error::RuleConfigMismatchedType(key.clone().into(), "Vec<String>".into())
                })
                .and_then(|vec| {
                    vec.iter()
                        .map(|v| {
                            v.as_str().map(|s| Cow::Borrowed(s)).ok_or_else(|| {
                                Error::RuleConfigMismatchedType(
                                    key.clone().into(),
                                    "Vec<String>".into(),
                                )
                            })
                        })
                        .collect()
                }),
            Self::Value(vec) => Ok(vec.iter().map(|s| Cow::Borrowed(s.as_str())).collect()),
        }
    }
}