use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;
use twilight_model::gateway::event::DispatchEvent;

use crate::model::{Action, Condition, Context, Trigger};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Rule {
    pub trigger: Trigger,
    pub conditions: Condition,
    /// Actions are executed sequentially if condition passes
    pub actions: Vec<Action>,
}

impl Rule {
    pub async fn check_event(
        &self,
        event: &DispatchEvent,
        context: &Context,
    ) -> Option<Result<bool, Box<dyn Error>>> {
        // if event.kind() != self.trigger {
        //     return None;
        // }

        Some(self.conditions.check_event(event, context).await)
    }
}
