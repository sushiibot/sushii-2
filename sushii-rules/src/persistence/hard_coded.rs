use crate::error::Result;
use crate::model::{constraint::*, Action, Condition, Rule, Trigger};
use lingua::Language;
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
                        constraint: Constraint::Message(MessageConstraint::Author(
                            UserConstraint::Id(IntegerConstraint::Equals(150443906511667200)),
                        )),
                    },
                    Condition::Condition {
                        constraint: Constraint::Message(MessageConstraint::Content(
                            StringConstraint::IsNotLanguage(Language::English),
                        )),
                    },
                ],
            },
            actions: vec![Action::Reply {
                content: "English only!".to_string(),
            }],
        }];

        Ok(rules)
    }
}
