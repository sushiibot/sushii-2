use crate::model::Status;
use std::ops::Deref;

pub struct ConditionResult {
    /// Name of condition related to this result
    name: String,
    // If condition is successful
    success: bool,
}

impl Deref for ConditionResult {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.success
    }
}
