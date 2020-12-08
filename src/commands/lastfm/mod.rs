use serenity::framework::standard::macros::group;

pub mod np;

use self::np::*;

#[group]
#[commands(np)]
pub struct LastFm;
