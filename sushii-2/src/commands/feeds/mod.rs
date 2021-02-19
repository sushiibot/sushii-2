use serenity::framework::standard::macros::group;

mod add;
mod delete;
mod list;

use self::{add::*, delete::*, list::*};

#[group]
#[commands(add, delete, list)]
#[prefixes("feed", "feeds")]
pub struct Feeds;
