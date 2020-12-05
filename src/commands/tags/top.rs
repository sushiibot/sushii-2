use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::model::sql::*;

fn fmt_tags(tags: &Vec<Tag>) -> String {
    let mut s = String::new();

    for tag in tags {
        let _ = writeln!(s, "{} - {}", tag.use_count, tag.tag_name);
    }

    s
}

#[command]
async fn top(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx, "Error: Not in guild").await?;
            return Ok(());
        }
    };

    let tags = Tag::get_top_used(&ctx, guild_id, 10).await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Server Tags (Most Used)"));
                e.description(fmt_tags(&tags));

                e
            });

            m
        })
        .await?;

    Ok(())
}
