use serenity::framework::standard::{macros::command, Args, CommandResult, Delimiter};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashSet;

use super::SETTINGS_GROUP;
use crate::commands::help::HELP_CMD;

#[command]
async fn default(ctx: &Context, msg: &Message) -> CommandResult {
    let args = Args::new("settings", &[Delimiter::Single(' ')]);
    (HELP_CMD.fun)(
        ctx,
        msg,
        args,
        &HELP_CMD.options,
        &[&SETTINGS_GROUP],
        HashSet::new(),
    )
    .await?;

    Ok(())
}
