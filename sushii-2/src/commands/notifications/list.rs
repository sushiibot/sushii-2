use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashMap;
use std::fmt::Write;

use crate::model::sql::*;

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let notis = Notification::user_notifications(ctx, msg.author.id).await?;

    if notis.is_empty() {
        msg.channel_id
            .say(&ctx, "Error: You have no notifications set")
            .await?;

        return Ok(());
    };

    let mut s = String::new();

    let mut guild_names: HashMap<u64, String> = HashMap::new();

    for noti in notis {
        let guild_name = if noti.guild_id != 0 {
            match guild_names.get(&(noti.guild_id as u64)) {
                Some(n) => n,
                None => {
                    let name = ctx
                        .cache
                        .guild_field(noti.guild_id as u64, |g| g.name.clone())
                        .await
                        .unwrap_or_else(|| "Unknown guild".into());

                    guild_names.insert(noti.guild_id as u64, name);
                    // Get from hashmap so don't have to clone
                    guild_names.get(&(noti.guild_id as u64)).unwrap()
                }
            }
        } else {
            "global"
        };

        writeln!(s, "{} - `{}`", guild_name, noti.keyword)?;
    }

    let res = msg
        .author
        .dm(ctx, |m| {
            m.embed(|e| {
                e.title("Notification Keywords");
                e.description(s);

                e
            })
        })
        .await;

    if res.is_err() {
        // Not discord reply since original message is deleted
        msg.reply_mention(
            ctx,
            "Failed to send you a DM, I can't send you notifications :( \
            Check if you have them enabled!",
        )
        .await?;
    } else {
        msg.channel_id.say(
            ctx,
            ":mailbox_with_mail: Sent a DM!",
        )
        .await?;

    }

    Ok(())
}
