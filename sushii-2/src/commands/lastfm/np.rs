use lastfm_rs::user::recent_tracks::Track;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;

use crate::keys::*;
use crate::model::sql::*;
use crate::utils::{text::escape_markdown, user::parse_id};

#[command]
#[only_in("guild")]
async fn np(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let recent_tracks = fm_client
        .recent_tracks(&lastfm_username)
        .await
        .with_limit(2)
        .send()
        .await?;

    metrics::increment_counter!("lastfm_api_queries", "endpoint" => "user.getRecentTracks");

    let track = match recent_tracks.tracks.first() {
        Some(t) => t,
        None => {
            msg.reply(ctx, "Error: No recent tracks were found").await?;

            return Ok(());
        }
    };

    let previous_track = recent_tracks.tracks.get(2);

    let now_playing = track
        .attrs
        .as_ref()
        .map_or(false, |a| a.now_playing == "true");
    let field_title = if now_playing {
        "Now listening to"
    } else {
        "Last listened to"
    };

    let thumbnail_url = track.images.get(2).map_or_else(
        || "https://i.imgur.com/oYm77EU.jpg",
        |i| i.image_url.as_str(),
    );

    let s = format_track(&track);
    let s_prev = previous_track.map(format_track);

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.colour(0xb90000);
                e.author(|a| {
                    a.icon_url("https://i.imgur.com/C7u8gqg.jpg");
                    a.name(&recent_tracks.attrs.user);

                    a
                });
                e.field(field_title, s, false);
                e.thumbnail(thumbnail_url);

                if let Some(s_prev) = s_prev {
                    e.field("Previous Track", s_prev, false);
                }

                e.footer(|f| {
                    f.text(format!(
                        "Total Tracks: {} • Powered by AudioScrobbler",
                        recent_tracks.attrs.total
                    ));

                    f
                });

                e
            });

            m
        })
        .await?;

    Ok(())
}

fn format_track(track: &Track) -> String {
    let mut s = String::new();

    let _ = write!(
        s,
        "{} - [{}]({})",
        escape_markdown(&track.artist.name),
        escape_markdown(&track.name),
        &track.url,
    );

    if !track.album.name.is_empty() {
        let _ = write!(s, "\nFrom {}", &track.album.name);
    }

    s
}
