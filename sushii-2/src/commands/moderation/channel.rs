use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("sm")]
#[required_permissions("MANAGE_GUILD")]
async fn slowmode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let slowmode_rate = match args.single::<u64>() {
        Ok(n) => n,
        Err(_) => {
            msg.channel_id
                .say(
                    ctx,
                    "Error: Invalid slowmode seconds, must be between 0 and 120 (inclusive)",
                )
                .await?;

            return Ok(());
        }
    };

    if slowmode_rate > 120 {
        msg.channel_id
            .say(
                ctx,
                "Error: Invalid slowmode seconds, must be between 0 and 120 (inclusive)",
            )
            .await?;

        return Ok(());
    }

    msg.channel_id
        .edit(ctx, |c| c.slow_mode_rate(slowmode_rate))
        .await?;

    msg.channel_id
        .say(
            ctx,
            format!(
                "Updated slowmode in {} to {} seconds",
                msg.channel_id.mention(),
                slowmode_rate
            ),
        )
        .await?;

    Ok(())
}
