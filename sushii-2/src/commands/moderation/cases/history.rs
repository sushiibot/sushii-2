use chrono::Utc;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_username;
use std::collections::HashMap;
use std::fmt::Write;

use crate::model::sql::ModLogEntry;
use crate::utils::duration::format_duration;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn history(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use </history:996259097441738763> now :)")
        .await?;

    return Ok(());
}
