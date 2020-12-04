use serenity::framework::standard::macros::group;

mod add;
mod edit;
mod get;
mod info;
mod list;

use self::{add::*, edit::*, get::*, info::*, list::*};

#[group]
#[commands(add, edit, info, get, rename, random, list)]
#[only_in("guild")]
#[prefix("tag")]
#[default_command(get)]
pub struct Tags;
