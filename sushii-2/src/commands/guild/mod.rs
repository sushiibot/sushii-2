use serenity::framework::standard::macros::group;

mod first;
mod leaderboard;
mod serverinfo;

use self::{first::*, leaderboard::*, serverinfo::*};

#[group]
#[commands(serverinfo, leaderboard, first)]
pub struct Guild;
