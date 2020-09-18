use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};
use std::collections::HashSet;

use serenity::prelude::*;

#[help]
#[individual_command_tip = "Pass a command as argument to get more information about a specific command."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "->"]
#[lacking_permissions = "Hide"]
#[embed_success_colour(ROHRKATZE_BLUE)]
async fn help_cmd(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
