use serenity::framework::standard::macros::group;

mod leaderboard;
mod serverinfo;

use self::{leaderboard::*, serverinfo::*};

#[group]
#[commands(serverinfo, leaderboard)]
pub struct Guild;
