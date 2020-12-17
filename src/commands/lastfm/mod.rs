use serenity::framework::standard::macros::group;

mod np;
mod recent;
mod set;

use self::{np::*, recent::*, set::*};

#[group]
#[prefix("fm")]
#[default_command(np)]
#[commands(np, set, recent)]
pub struct LastFm;
