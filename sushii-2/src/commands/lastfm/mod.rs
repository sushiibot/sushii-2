use serenity::framework::standard::macros::group;

mod loved;
mod np;
mod recent;
mod set;
mod topartists;

use self::{loved::*, np::*, recent::*, set::*, topartists::*};

#[group]
#[prefix("fm")]
#[default_command(np)]
#[commands(np, set, recent, loved, topartists)]
pub struct LastFm;
