use async_recursion::async_recursion;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::Result;
use crate::model::{Constraint, Event, RuleContext};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Condition {
    /// # And
    /// Require *all** conditions to pass before running actions
    And { and: Vec<Condition> },
    /// # Or
    /// Require *at least one** conditions to pass before running actions
    Or { or: Vec<Condition> },
    /// # Not
    /// Require condition to fail before running actions
    Not { not: Box<Condition> },
    /// # At least n
    /// Require at least a given number of conditions to pass before running actions
    AtLeast {
        min_count: usize,
        conditions: Vec<Condition>,
    },
    /// # Condition
    /// Conditions for the rule to run
    Condition {
        #[serde(flatten)]
        constraint: Constraint,
    },
}

impl Condition {
    #[async_recursion]
    pub async fn check_event(&self, event: Arc<Event>, context: &RuleContext) -> Result<bool> {
        match *self {
            Condition::And { ref and } => {
                for child in and.iter() {
                    if !child.check_event(event.clone(), context).await? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            Condition::Not { not: ref c } => {
                return c.check_event(event.clone(), context).await.map(|r| !r);
            }
            Condition::Or { ref or } => {
                for child in or.iter() {
                    if child.check_event(event.clone(), context).await? {
                        return Ok(true);
                    }
                }

                Ok(false)
            }
            Condition::AtLeast {
                min_count,
                ref conditions,
            } => {
                let mut count = 0;

                for child in conditions.iter() {
                    if child.check_event(event.clone(), context).await? {
                        count += 1;
                    }
                }

                Ok(count >= min_count)
            }
            Condition::Condition { ref constraint } => constraint.check_event(event, context).await,
        }
    }
}
