use serde::Serialize;
use sushii_model::model::sql::RuleGauge;
use twilight_model::gateway::event::DispatchEvent;

use crate::model::Trigger;

#[derive(Debug, Clone, Serialize)]
pub enum Event {
    /// Gateway dispatch event by itself
    Twilight(DispatchEvent),
    /// Counter along with the event that caused this counter change
    Counter(RuleGauge, DispatchEvent),
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
            Self::Counter(_, _) => Trigger::Counter,
        }
    }
}
