use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserLevelProgress {
    pub level: i64,
    pub next_level_xp_required: i64,
    pub next_level_xp_progress: i64,
    pub next_level_xp_percentage: u64,
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
            level,
            next_level_xp_required,
            next_level_xp_progress,
            next_level_xp_percentage,
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
