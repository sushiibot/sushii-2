use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::keys::*;
use crate::model::sql::*;
use crate::utils::user::parse_id;

#[command]
#[only_in("guild")]
async fn profile(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_id = if args.is_empty() {
        msg.author.id
    } else {
        match args.single::<String>().ok().and_then(parse_id) {
            Some(id) => UserId(id),
            None => {
                msg.reply(&ctx, "Error: Invalid user given").await?;

                return Ok(());
            }
        }
    };

    let lastfm_username = match UserData::from_id(ctx, target_id)
        .await?
        .and_then(|d| d.lastfm_username)
    {
        Some(d) => d,
        None => {
            msg.reply(
                ctx,
                "Error: No Last.fm username saved, use `fm set [username]`",
            )
            .await?;

            return Ok(());
        }
    };

    let reqwest_client = ctx
        .data
        .read()
        .await
        .get::<ReqwestContainer>()
        .cloned()
        .unwrap();

    let sushii_conf = SushiiConfig::get(ctx).await;

    let mut fm_client =
        lastfm_rs::Client::from_reqwest_client(reqwest_client.clone(), &sushii_conf.lastfm_key);

    let user_info = fm_client
        .user_info(&lastfm_username)
        .await
        .send()
        .await
        .map(|u| u.user)?;

    let thumbnail_url = user_info.images.get(2).map_or_else(
        || "https://i.imgur.com/6ATbUNw.jpg",
        |i| i.image_url.as_str(),
    );

    let mut s = String::new();

    writeln!(s, "**Total Scrobbles:** {}", user_info.scrobbles)?;
    writeln!(s, "**Registered:** {}", user_info.registered.date.format("%Y-%m-%dT%H:%M:%S"))?;

    if !user_info.country.is_empty() {
        writeln!(s, "**Country:** {}", user_info.country)?;
    }

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.colour(0xb90000);
                e.author(|a| {
                    a.icon_url("https://i.imgur.com/C7u8gqg.jpg");
                    a.name(&user_info.username);
                    a.url(&user_info.url);

                    a
                });
                e.thumbnail(thumbnail_url);

                e.description(s);

                e.footer(|f| {
                    f.text("Powered by AudioScrobbler");

                    f
                });

                e
            });

            m
        })
        .await?;

    Ok(())
}
