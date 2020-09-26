use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::model::sql::*;

#[command]
#[aliases("sendhelp", "send info", "send help")]
async fn sendinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let conf = GuildConfig::from_msg_or_respond(&ctx, msg).await?;

    let role_conf: GuildRoles = match conf.role_config {
        Some(c) => serde_json::from_value(c)?,
        None => {
            msg.channel_id
                .say(&ctx, "Error: There is no role config set")
                .await?;

            return Ok(());
        }
    };

    let role_channel = match conf.role_channel {
        Some(c) => c as u64,
        None => {
            msg.channel_id
                .say(&ctx, "Error: There is no role channel set")
                .await?;

            return Ok(());
        }
    };

    if role_conf.groups.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: Role config contains no role groups")
            .await?;
        return Ok(());
    }

    let s = format!(
        "Add a role: `+name`
Remove a role: `-name`

Multiple actions can be combined:
`+first +second -third`

Remove all roles: `clear` or `reset`

{}
**Examples**
{}",
        role_conf,
        role_conf.get_examples_string()
    );

    if let Err(e) = ChannelId(role_channel)
        .send_message(&ctx.http, |m| {
            // Just some zws's to make it easier to tell which message is a role help message
            m.content("\u{200B}".repeat(5));

            m.embed(|e| {
                e.title("Server Roles");
                e.description(&s);
                e.color(0x7596ff);

                e
            })
        })
        .await
    {
        tracing::warn!("Failed to send role info message: {}", e);
        msg.channel_id
            .say(&ctx.http, "Error: Failed to send role help message")
            .await?;

        return Ok(());
    }

    msg.channel_id
        .say(
            &ctx.http,
            format!("Sent role info message to <#{}>", role_channel),
        )
        .await?;

    Ok(())
}
