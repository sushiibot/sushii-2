use serenity::framework::standard::macros::group;

mod add;
mod list;

use self::{add::*, list::*};

#[group]
#[commands(add, list)]
#[prefixes("feed", "feeds")]
pub struct Feeds;
