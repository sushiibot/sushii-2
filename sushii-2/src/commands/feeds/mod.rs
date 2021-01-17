use serenity::framework::standard::macros::group;

mod add;

use self::add::*;

#[group]
#[commands(add)]
#[prefixes("feed", "feeds")]
pub struct Feeds;
