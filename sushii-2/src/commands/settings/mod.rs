use serenity::framework::standard::macros::group;

mod default;
mod disable_channel;
mod list;
mod mute;

use self::{default::*, disable_channel::*, list::*, mute::*};

#[group]
#[commands(list, mute, disablechannel, enablechannel, disabledchannels)]
#[description("Guild settings, requires MANAGE_GUILD permissions")]
#[prefix("settings")]
#[only_in("guild")]
#[default_command(default)]
#[required_permissions("MANAGE_GUILD")]
pub struct Settings;
