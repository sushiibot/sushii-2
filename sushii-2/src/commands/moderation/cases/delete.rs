use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use sushii_model::model::sql::ModLogEntry;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
#[aliases("clearcase", "casedelete", "uncase")]
async fn deletecase(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id.0,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let case_id = match args.single::<u64>() {
        Ok(id) => id,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Error: Give a valid case ID")
                .await?;

            return Ok(());
        }
    };

    let case = match ModLogEntry::from_case_id(ctx, guild_id, case_id).await? {
        Some(c) => c,
        None => {
            msg.channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Error: Case `#{}` doesn't exist, give a valid case ID",
                        case_id
                    ),
                )
                .await?;

            return Ok(());
        }
    };

    case.delete(ctx).await?;

    msg.channel_id
        .say(&ctx.http, format!("Case `#{}` has been deleted", case_id))
        .await?;

    Ok(())
}
