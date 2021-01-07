use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::utils::user::parse_id;

#[command]
#[aliases("av")]
#[only_in("guild")]
async fn avatar(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let target_str = args.rest();

    let target_id = match parse_id(target_str) {
        Some(id) => UserId(id),
        None => {
            if !target_str.is_empty() {
                msg.channel_id
                    .say(ctx, "Error: Invalid user given.")
                    .await?;

                return Ok(());
            }

            // If empty use self
            msg.author.id
        }
    };

    let user = target_id.to_user(ctx).await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(&user.tag());
                    a.url(user.face());

                    a
                });

                e.image(user.face());

                e
            })
        })
        .await?;

    Ok(())
}
