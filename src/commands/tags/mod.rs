use serenity::framework::standard::macros::group;

mod add;
mod edit;
mod get;
mod info;
mod list;
mod search;
mod top;

use self::{add::*, edit::*, get::*, info::*, list::*, search::*, top::*};

#[group]
#[commands(add, edit, info, get, rename, random, list, search, top)]
#[only_in("guild")]
#[prefix("tag")]
#[default_command(get)]
pub struct Tags;
