use serenity::framework::standard::macros::group;

mod loved;
mod np;
mod profile;
mod recent;
mod set;
mod topartists;

use self::{loved::*, np::*, profile::*, recent::*, set::*, topartists::*};

#[group]
#[prefix("fm")]
#[default_command(np)]
#[commands(np, set, recent, loved, topartists, profile)]
pub struct LastFm;
