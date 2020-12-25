use serde::{Deserialize, Serialize};
#[cfg(feature = "graphql")]
use juniper::GraphQLObject;

use crate::model::BigInt;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(
    feature = "graphql",
    graphql(description = "A user's level progress"),
    derive(GraphQLObject)
)]
pub struct UserLevelProgress {
    pub level: BigInt,
    pub next_level_xp_required: BigInt,
    pub next_level_xp_progress: BigInt,
    pub next_level_xp_percentage: BigInt,
}

impl UserLevelProgress {
    pub fn from_xp(xp: i64) -> Self {
        let level = get_level(xp);

        // Total amount of XP to get to current level
        let curr_level_total_xp = next_level(level);
        // Total amount of XP to get to next level
        let next_level_total_xp = next_level(level + 1);

        // How much total XP needed to get from current level to next
        let next_level_xp_required = next_level_total_xp - curr_level_total_xp;
        // How much XP needed left considering current XP
        let next_level_xp_remaining = next_level_total_xp - xp;
        // How much XP gained only in current level
        let next_level_xp_progress = next_level_xp_required - next_level_xp_remaining;

        let next_level_xp_percentage =
            ((next_level_xp_progress as f64 / next_level_xp_required as f64) * 100.0) as u64;

        Self {
            level: level.into(),
            next_level_xp_required: next_level_xp_required.into(),
            next_level_xp_progress: next_level_xp_progress.into(),
            next_level_xp_percentage: next_level_xp_percentage.into(),
        }
    }
}

pub fn next_level(level: i64) -> i64 {
    50 * (level.pow(2)) - (50 * level)
}

pub fn get_level(xp: i64) -> i64 {
    let mut level = 0;
    while next_level(level + 1) <= xp {
        level += 1;
    }

    level
}
