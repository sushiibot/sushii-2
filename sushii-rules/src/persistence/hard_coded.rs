use lingua::Language;
use sqlx::types::Uuid;
use std::collections::HashMap;
use twilight_model::gateway::event::EventType;

use sushii_model::model::sql::RuleScope;

use super::RuleStore;
use crate::error::Result;
use crate::model::{constraint::*, Action, Condition, Rule, RuleSet, Trigger};

// Just for testing other functionality right now, main store should be postgres
#[derive(Clone, Debug)]
pub struct HardCodedStore;

impl HardCodedStore {
    pub fn new() -> Self {
        Self
    }
}

impl RuleStore for HardCodedStore {
    fn get_guild_rule_sets(&self, guild_id: u64) -> Result<Vec<RuleSet>> {
        if guild_id != 167058919611564043 {
            return Ok(Vec::new());
        }

        let rules = vec![
            Rule {
                id: Uuid::nil(),
                name: "Language counter".into(),
                enabled: true,
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
                actions: vec![Action::AddCounter {
                    name: "language warning".to_string(),
                    scope: RuleScope::User,
                }],
            },
            Rule {
                id: Uuid::nil(),
                name: "Language warning".into(),
                enabled: true,
                trigger: Trigger::Counter,
                conditions: Condition::Condition {
                    constraint: Constraint::Counter(CounterConstraint {
                        name: "language warning".to_string(),
                        scope: RuleScope::User,
                        value: CounterValueConstraint::LessThanOrEqual(3),
                    }),
                },
                actions: vec![Action::Reply {
                    content:
                        "**Warning {{ trigger.counter.value }} / 3**: Please keep chat in English!"
                            .to_string(),
                }],
            },
            Rule {
                id: Uuid::nil(),
                name: "Language mute".into(),
                enabled: true,
                trigger: Trigger::Counter,
                conditions: Condition::Condition {
                    constraint: Constraint::Counter(CounterConstraint {
                        name: "language warning".to_string(),
                        scope: RuleScope::User,
                        value: CounterValueConstraint::GreaterThan(3),
                    }),
                },
                actions: vec![
                    Action::Mute {
                        // 1 hour mute
                        duration: Some(60 * 60),
                        reason: Some("Rule 6, Repeated chat not in English ".to_string()),
                    },
                    Action::Reply {
                        content: "muted lol".to_string(),
                    },
                    Action::ResetCounter {
                        name: "language warning".to_string(),
                        scope: RuleScope::User,
                    },
                ],
            },
        ];

        let rule_set = RuleSet {
            id: Uuid::nil(),
            guild_id: 167058919611564043,
            name: "Language auto-mod".into(),
            description: Some("Auto mutes users speaking non-English languages".into()),
            enabled: true,
            editable: true,
            author: 150443906511667200,
            category: Some("Auto-moderator".into()),
            config: HashMap::new(),
            rules: rules,
        };

        Ok(vec![rule_set])
    }
}
