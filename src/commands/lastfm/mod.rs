use serenity::framework::standard::macros::group;

pub mod np;
pub mod set;

use self::{np::*, set::*};

#[group]
#[prefix("fm")]
#[default_command(np)]
#[commands(np, set)]
pub struct LastFm;
