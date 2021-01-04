use serenity::framework::standard::macros::group;

mod loved;
mod np;
mod recent;
mod set;

use self::{loved::*, np::*, recent::*, set::*};

#[group]
#[prefix("fm")]
#[default_command(np)]
#[commands(np, set, recent, loved)]
pub struct LastFm;
