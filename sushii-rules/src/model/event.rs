use serde::Serialize;
use sushii_model::model::sql::RuleGauge;
use twilight_model::channel::message::Message;
use twilight_model::gateway::event::DispatchEvent;

use crate::model::Trigger;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Event {
    /// Gateway dispatch event by itself
    Twilight(DispatchEvent),
    /// Counter along with the event that caused this counter change
    Counter {
        counter: RuleGauge,
        original_event: DispatchEvent,
    },
    /// Timer
    LevelUp {
        /// ID of the user
        user_id: u64,
        // Boxed because of clippy
        // https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
        /// Message that triggered this event
        message: Box<Message>,
        /// New user level
        level: u64,
        /// Current user XP
        xp: u64,
        /// Previous user level
        old_level: u64,
    },
}

impl From<DispatchEvent> for Event {
    fn from(event: DispatchEvent) -> Self {
        Self::Twilight(event)
    }
}

impl Event {
    pub fn kind(&self) -> Trigger {
        match self {
            Self::Twilight(event) => event.kind().into(),
            Self::Counter { .. } => Trigger::Counter,
            Self::LevelUp { .. } => Trigger::LevelUp,
        }
    }
}
