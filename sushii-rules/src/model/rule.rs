use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
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
        event: Arc<DispatchEvent>,
        context: &Context,
    ) -> Result<bool, Box<dyn Error>> {
        // if event.kind() != self.trigger {
        //     return None;
        // }

        let passes_conditions = self.conditions.check_event(event.clone(), context).await?;

        if !dbg!(passes_conditions) {
            return Ok(false);
        }

        for action in &self.actions {
            action.execute(event.clone(), &context).await?;
        }

        Ok(true)
    }
}
