use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::parse_mention;

use crate::keys::*;
use crate::model::sql::*;

#[command]
#[only_in("guild")]
async fn np(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let target_id = if args.is_empty() {
        msg.author.id
    } else {
        match args
            .single::<String>()
            .ok()
            .and_then(|id_str| id_str.parse::<u64>().ok().or_else(|| parse_mention(id_str)))
        {
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

    let recent_tracks = fm_client
        .recent_tracks(&lastfm_username)
        .await
        .with_limit(1)
        .send()
        .await?;

    let track = match recent_tracks.tracks.first() {
        Some(t) => t,
        None => {
            msg.reply(ctx, "Error: No recent tracks were found").await?;

            return Ok(());
        }
    };

    let now_playing = track
        .attrs
        .as_ref()
        .map_or(false, |a| a.now_playing == "true");
    let field_title = if now_playing {
        "Now listening to"
    } else {
        "Last listened to"
    };

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.icon_url("https://i.imgur.com/C7u8gqg.jpg");
                    a.name(&recent_tracks.attrs.user);

                    a
                });
                e.field(
                    field_title,
                    format!("{} - [{}]({})", &track.artist.name, &track.name, &track.url),
                    false,
                );
                e.field("Album", &track.album.name, false);

                e
            });

            m
        })
        .await?;

    Ok(())
}
