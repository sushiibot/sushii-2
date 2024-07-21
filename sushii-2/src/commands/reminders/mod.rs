use serenity::framework::standard::macros::group;

mod add;

use self::{add::*};

#[group]
#[commands(add)]
#[default_command(add)]
#[prefixes("reminder", "remindme", "remind", "remi")]
pub struct Reminders;
