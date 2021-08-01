use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn stickerinfo(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.sticker_items.is_empty() {
        msg.channel_id.say(&ctx, "No stickers in message").await?;
        return Ok(());
    }

    let sticker = msg.sticker_items.first().unwrap().to_sticker(&ctx).await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(&sticker.name);

                // If we have the `MANAGE_EMOJIS_AND_STICKERS` permission,
                // we can also view who created the sticker. BUT we need to use
                // the separate guild sticker endpoint so this will never
                // show up
                if let Some(ref user) = sticker.user {
                    e.author(|a| {
                        a.name(user.tag());
                        a.icon_url(user.face());
                        a.url(user.face());

                        a
                    });
                }

                if let Some(ref description) = sticker.description {
                    e.description(description);
                }
                if let Some(ref url) = sticker.image_url() {
                    e.url(url);

                    // Can't display Lottie files here
                    if url.ends_with("png") {
                        e.image(url);
                    }
                }

                e.field("Tags", sticker.tags.join(", "), false);
                e.field("Format", format!("{:?}", &sticker.format_type), false);
                e.field("Type", format!("{:?}", &sticker.kind), false);

                if let Some(ref guild_id) = sticker.guild_id {
                    e.field("Guild ID", guild_id.to_string(), false);
                }

                e.footer(|f| f.text(format!("ID {}", sticker.id)));

                e
            });

            m
        })
        .await?;

    Ok(())
}
