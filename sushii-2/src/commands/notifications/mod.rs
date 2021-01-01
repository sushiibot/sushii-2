use serenity::framework::standard::macros::group;

pub mod add;

use self::add::*;

#[group]
#[commands(add)]
#[prefixes("noti", "notification")]
pub struct Notifications;
