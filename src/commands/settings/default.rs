use serenity::framework::standard::{macros::command, CommandResult, Args, Delimiter};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashSet;

use crate::commands::help::HELP_CMD;
use super::SETTINGS_GROUP;

#[command]
async fn default(ctx: &Context, msg: &Message) -> CommandResult {
    let args = Args::new("settings", &[Delimiter::Single(' ')]);
    (HELP_CMD.fun)(ctx, msg, args, &HELP_CMD.options, &[&SETTINGS_GROUP], HashSet::new()).await?;

    Ok(())
}