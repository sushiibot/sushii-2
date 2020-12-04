use serenity::framework::standard::macros::group;

mod add;
mod edit;
mod get;
mod info;

use self::{add::*, edit::*, get::*, info::*};

#[group]
#[commands(add, edit, info, get, rename, random)]
#[only_in("guild")]
#[prefix("tag")]
#[default_command(get)]
pub struct Tags;
