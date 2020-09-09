use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_role;

use crate::model::sql::*;

#[command]
#[num_args(1)]
async fn muterole(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = match msg.guild(&ctx.cache).await {
        Some(g) => g,
        None => {
            let _ = msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    let mut conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let role_str = args.rest();

    let role_id = parse_role(role_str)
        .or_else(|| role_str.parse::<u64>().ok())
        .or_else(|| guild.roles
            .values()
            .find(|&x| x.name == role_str)
            .map(|x| x.id.0)
        );

    if let Some(id) = role_id {
        conf.mute_role.replace(id as i64);
        conf.save(&ctx).await?;

        msg.channel_id
            .say(
                &ctx.http,
                format!("Updated mute role to ID {}", id),
            )
            .await?;
    } else {
        
        msg.channel_id
            .say(
                &ctx.http,
                "Invalid role, give a role name, mention, or ID",
            )
            .await?;
    }


    Ok(())
}
