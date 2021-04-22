use async_recursion::async_recursion;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use twilight_model::gateway::event::DispatchEvent;

use crate::error::Result;
use crate::model::{Constraint, RuleContext};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Condition {
    And {
        and: Vec<Condition>,
    },
    Or {
        or: Vec<Condition>,
    },
    Not {
        not: Box<Condition>,
    },
    AtLeast {
        min_count: usize,
        conditions: Vec<Condition>,
    },
    Condition {
        #[serde(flatten)]
        constraint: Constraint,
    },
}

impl Condition {
    #[async_recursion]
    pub async fn check_event(
        &self,
        event: Arc<DispatchEvent>,
        context: &RuleContext,
    ) -> Result<bool> {
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
