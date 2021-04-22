use crate::error::Result;
use crate::model::{constraint::*, Action, Condition, Rule, Trigger};
use twilight_model::gateway::event::EventType;

use super::RuleStore;

#[derive(Clone, Debug)]
pub struct HardCodedStore;

impl HardCodedStore {
    pub fn new() -> Self {
        Self
    }
}

impl RuleStore for HardCodedStore {
    fn get_guild_rules(&self, guild_id: u64) -> Result<Vec<Rule>> {
        let rules = vec![Rule {
            trigger: Trigger::Twilight(EventType::MessageCreate),
            conditions: Condition::And {
                and: vec![
                    Condition::Condition {
                        constraint: Constraint::Message(MessageConstraint::Content(
                            StringConstraint::Equals("!ping".to_string()),
                        )),
                    },
                    Condition::Condition {
                        constraint: Constraint::Message(MessageConstraint::Author(
                            UserConstraint::Username(StringConstraint::Equals("tzuwy".to_string())),
                        )),
                    },
                ],
            },
            actions: vec![Action::Reply {
                content: "Pong!".to_string(),
            }],
        }];

        Ok(rules)
    }
}
