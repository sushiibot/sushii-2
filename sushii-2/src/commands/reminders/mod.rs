use serenity::framework::standard::macros::group;

mod add;
mod list;

use self::{add::*, list::*};

#[group]
#[commands(add, list)]
#[default_command(add)]
#[prefixes("reminder", "remind", "remi")]
pub struct Reminders;
