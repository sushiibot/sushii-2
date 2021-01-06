use serenity::framework::standard::macros::group;

mod loved;
mod np;
mod recent;
mod set;
mod topartists;
mod profile;

use self::{loved::*, np::*, recent::*, set::*, topartists::*, profile::*};

#[group]
#[prefix("fm")]
#[default_command(np)]
#[commands(np, set, recent, loved, topartists, profile)]
pub struct LastFm;
