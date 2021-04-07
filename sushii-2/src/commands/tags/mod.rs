use serenity::framework::standard::macros::group;

mod add;
mod author;
mod edit;
mod get;
mod info;
mod list;
mod search;
mod top;

use self::{add::*, author::*, edit::*, get::*, info::*, list::*, search::*, top::*};

#[group]
#[commands(add, author, edit, delete, info, get, rename, random, list, search, top)]
#[only_in("guild")]
#[prefixes("tag", "t")]
#[default_command(get)]
pub struct Tags;
