use futures::StreamExt;
use serenity::collector::reaction_collector::ReactionAction;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;
use std::sync::Arc;
use std::time::Duration;

use crate::error::Error;
use crate::keys::*;
use crate::model::sql::*;
use crate::utils::user::parse_id;

#[command]
#[only_in("guild")]
async fn recent(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

    let typing = msg.channel_id.start_typing(&ctx.http)?;
    let mut recent_tracks = fm_client
        .recent_tracks(&lastfm_username)
        .await
        .with_page(1)
        .with_limit(10)
        .send()
        .await
        .map(Arc::new)?;

    metrics::increment_counter!("lastfm_api_queries", "endpoint" => "user.getRecentTracks");

    if recent_tracks.tracks.is_empty() {
        msg.reply(ctx, "Error: No recent tracks were found").await?;

        return Ok(());
    };

    let mut recent_tracks_cache = Vec::new();
    recent_tracks_cache.push(Arc::clone(&recent_tracks));

    let mut page = 1;
    // API returns strings..
    let total_pages = recent_tracks.attrs.total_pages.parse::<usize>()?;

    let mut s = String::new();

    for track in &recent_tracks.tracks {
        let _ = writeln!(
            s,
            "{} - [{}]({})",
            &track.artist.name, &track.name, &track.url
        );
    }

    let thumbnail_url = recent_tracks
        .tracks
        .iter()
        .find(|track| track.images.get(2).is_some())
        .and_then(|t| t.images.get(2))
        .map_or_else(
            || "https://i.imgur.com/oYm77EU.jpg",
            |i| i.image_url.as_str(),
        );

    let mut sent_msg = msg
        .channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.colour(0xb90000);
                e.author(|a| {
                    a.icon_url("https://i.imgur.com/C7u8gqg.jpg");
                    a.name(&recent_tracks.attrs.user);

                    a
                });
                e.title("Recent Tracks");
                e.description(s);
                e.thumbnail(thumbnail_url);
                e.footer(|f| {
                    f.text(format!("Page {}/{}", page, total_pages));
                    f
                });

                e
            });

            m.reactions(vec![
                ReactionType::Unicode("⬅️".into()),
                ReactionType::Unicode("➡️".into()),
            ]);

            m
        })
        .await?;
    typing.stop();

    while let Some(reaction_action) = sent_msg
        .await_reactions(ctx)
        .author_id(msg.author.id)
        .filter(|r| ["⬅️", "➡️"].iter().any(|u| r.emoji.unicode_eq(u)))
        .timeout(Duration::from_secs(45))
        .await
        .next()
        .await
    {
        if let ReactionAction::Added(ref r) = *reaction_action {
            // tracing::info!("offsets: {:?}", offsets);

            // Next page
            if r.emoji.unicode_eq("➡️") {
                // Ignore on last
                if page == total_pages {
                    r.delete(&ctx).await?;
                    continue;
                }

                page += 1;

                recent_tracks =
                    if let Some(track) = recent_tracks_cache.get(page - 1).map(Arc::clone) {
                        track
                    } else {
                        let new_recent_tracks = fm_client
                            .recent_tracks(&lastfm_username)
                            .await
                            .with_page(page)
                            .with_limit(10)
                            .send()
                            .await
                            .map(Arc::new)?;

                        metrics::increment_counter!("lastfm_api_queries", "endpoint" => "user.getRecentTracks");

                        recent_tracks_cache.push(Arc::clone(&new_recent_tracks));

                        new_recent_tracks
                    };
            } else if r.emoji.unicode_eq("⬅️") {
                // Ignore on first page
                if page == 1 {
                    r.delete(&ctx).await?;
                    continue;
                }

                page -= 1;

                // Page is 1 indexed, Vec::get() is 0 indexed
                recent_tracks = recent_tracks_cache
                    .get(page - 1)
                    .map(Arc::clone)
                    .ok_or_else(|| Error::Sushii("Missing loved_tracks page in cache".into()))?;
            }

            let thumbnail_url = recent_tracks
                .tracks
                .iter()
                .find(|track| track.images.get(2).is_some())
                .and_then(|t| t.images.get(2))
                .map_or_else(
                    || "https://i.imgur.com/oYm77EU.jpg",
                    |i| i.image_url.as_str(),
                );

            let mut s = String::new();
            for track in &recent_tracks.tracks {
                let _ = writeln!(
                    s,
                    "{} - [{}]({})",
                    &track.artist.name, &track.name, &track.url
                );
            }

            sent_msg
                .edit(ctx, |m| {
                    m.embed(|e| {
                        e.colour(0xb90000);
                        e.author(|a| {
                            a.icon_url("https://i.imgur.com/C7u8gqg.jpg");
                            a.name(&recent_tracks.attrs.user);

                            a
                        });
                        e.title("Recent Tracks");
                        e.description(s);
                        e.thumbnail(thumbnail_url);
                        e.footer(|f| {
                            f.text(format!("Page {}/{}", page, total_pages));
                            f
                        });

                        e
                    });

                    m
                })
                .await?;

            // Delete reaction after handling, so that user can react again
            r.delete(&ctx).await?;
        }
    }

    Ok(())
}
