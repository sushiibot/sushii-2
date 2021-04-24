use serenity::framework::standard::macros::group;

mod loved;
mod np;
mod profile;
mod recent;
mod set;
mod unset;
mod topartists;

use self::{loved::*, np::*, profile::*, recent::*, set::*, topartists::*, unset::*};

#[group]
#[prefix("fm")]
#[default_command(np)]
#[commands(np, set, unset, recent, loved, topartists, profile)]
pub struct LastFm;
