use serenity::framework::standard::macros::group;

mod add;
mod edit;
mod get;
mod info;
mod list;
mod search;

use self::{add::*, edit::*, get::*, info::*, list::*, search::*};

#[group]
#[commands(add, edit, info, get, rename, random, list, search)]
#[only_in("guild")]
#[prefix("tag")]
#[default_command(get)]
pub struct Tags;
