use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("firstmsg", "firstmessage")]
#[only_in("guild")]
async fn first(ctx: &Context, msg: &Message) -> CommandResult {
    let first_msg = msg
        .channel_id
        .messages(ctx, |r| r.after(MessageId(msg.channel_id.0)).limit(1))
        .await?;

    if let Some(first_msg) = first_msg.first() {
        let link = match msg.guild_id {
            Some(guild_id) => first_msg.link().replace("@me", &guild_id.0.to_string()),
            None => first_msg.link(),
        };

        msg.channel_id
            .say(ctx, format!("First message in this channel:\n{}", link))
            .await?;
    } else {
        msg.channel_id
            .say(ctx, "Error: Could not find the first message :(")
            .await?;
    }

    Ok(())
}
