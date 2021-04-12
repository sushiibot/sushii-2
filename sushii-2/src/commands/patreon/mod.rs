use serenity::framework::standard::macros::group;

mod patron;

use self::patron::*;

#[group]
#[commands(patron)]
pub struct Patreon;
