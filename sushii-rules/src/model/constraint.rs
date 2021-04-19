use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;
use twilight_model::gateway::event::DispatchEvent;

use crate::model::Context;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "operator", content = "value")]
pub enum StringConstraint {
    Equals(String),
    NotEquals(String),
    Contains(String),
    ContainsAll(Vec<String>),
    ContainsAny(Vec<String>),
    DoesNotContain(String),
    DoesNotContainAny(Vec<String>),
    In(Vec<String>),
    NotIn(Vec<String>),
    StartsWith(String),
    DoesNotStartsWith(String),
    EndsWith(String),
    DoesNotEndsWith(String),
}

impl StringConstraint {
    #[rustfmt::skip]
    pub fn check_string(&self, in_str: &str) -> bool {
        match self {
            Self::Equals(s) => {
                in_str == *s
            }
            Self::NotEquals(s) => {
                in_str != *s
            }
            Self::Contains(s) => {
                in_str.contains(s)
            }
            Self::ContainsAll(strs) => {
                strs.iter().all(|s| in_str.contains(s))
            },
            Self::ContainsAny(strs) => {
                strs.iter().any(|s| in_str.contains(s))
            },
            Self::DoesNotContain(s) => {
                !in_str.contains(s)
            },
            Self::DoesNotContainAny(strs) => {
                !strs.iter().all(|s| in_str.contains(s))
            },
            Self::In(strs) => {
                strs.iter().all(|s| s.contains(&in_str))
            }
            Self::NotIn(strs) => {
                !strs.iter().all(|s| s.contains(&in_str))
            }
            Self::StartsWith(s) => {
                in_str.starts_with(s)
            }
            Self::DoesNotStartsWith(s) => {
                !in_str.starts_with(s)
            }
            Self::EndsWith(s) => {
                in_str.ends_with(s)
            }
            Self::DoesNotEndsWith(s) => {
                !in_str.ends_with(s)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "field_type", content = "value")]
pub enum UserConstraint {
    Username(StringConstraint),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "field_type", content = "value")]
pub enum MessageConstraint {
    Content(StringConstraint),
    Author(UserConstraint),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "event_type", content = "value")]
pub enum Constraint {
    Message(MessageConstraint),
}

impl Constraint {
    pub async fn check_event(
        &self,
        event: &DispatchEvent,
        _context: &Context,
    ) -> Result<bool, Box<dyn Error>> {
        let val = match (self, event) {
            (
                Constraint::Message(MessageConstraint::Content(c)),
                DispatchEvent::MessageCreate(msg),
            ) => c.check_string(&msg.content),
            _ => unimplemented!(),
        };

        Ok(val)
    }
}
