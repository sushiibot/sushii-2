use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_channel;

#[command]
#[aliases("sm")]
#[required_permissions("MANAGE_GUILD")]
async fn slowmode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let slowmode_rate = match args.single::<u64>() {
        Ok(n) => n,
        Err(_) => {
            msg.reply(
                ctx,
                "Error: Invalid slowmode seconds, must be between 0 and 120 (inclusive)",
            )
            .await?;

            return Ok(());
        }
    };

    let target_channel_str = args.rest().trim();
    let target_channel = if target_channel_str.is_empty() {
        msg.channel_id
    } else {
        match target_channel_str
            .parse::<u64>()
            .ok()
            .or_else(|| parse_channel(target_channel_str))
        {
            Some(id) => ChannelId(id),
            None => {
                msg.reply(ctx, "Error: Invalid channel").await?;

                return Ok(());
            }
        }
    };

    let guild = match msg.guild(ctx).await {
        Some(g) => g,
        None => {
            msg.reply(ctx, "No guild found").await?;

            return Ok(());
        }
    };

    if !guild.channels.contains_key(&target_channel) {
        msg.reply(ctx, "Error: Channel is not in this guild or could not be found").await?;

        return Ok(());
    }

    if slowmode_rate > 120 {
        msg.channel_id
            .say(
                ctx,
                "Error: Invalid slowmode seconds, must be between 0 and 120 (inclusive)",
            )
            .await?;

        return Ok(());
    }

    target_channel
        .edit(ctx, |c| c.slow_mode_rate(slowmode_rate))
        .await?;

    msg.channel_id
        .say(
            ctx,
            format!(
                "Updated slowmode in {} to {} seconds",
                target_channel.mention(),
                slowmode_rate
            ),
        )
        .await?;

    Ok(())
}
