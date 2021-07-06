use crate::error::Result;
use crate::model::RuleSet;
use std::fmt;

// pub mod hard_coded;
// pub mod postgres;

// pub use hard_coded::HardCodedStore;

pub trait RuleStore: RuleStoreClone + fmt::Debug {
    /// Fetches all rules in a guild
    fn get_guild_rule_sets(&self, guild_id: u64) -> Result<Vec<RuleSet>>;
}

pub trait RuleStoreClone {
    fn clone_box(&self) -> Box<dyn RuleStore>;
}

impl<T> RuleStoreClone for T
where
    T: 'static + RuleStore + Clone + fmt::Debug,
{
    fn clone_box(&self) -> Box<dyn RuleStore> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn RuleStore> {
    fn clone(&self) -> Box<dyn RuleStore> {
        self.clone_box()
    }
}
