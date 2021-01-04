use futures::StreamExt;
use lastfm_rs::user::top_artists::Period;
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

#[command]
#[only_in("guild")]
async fn topartists(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let period = if args.is_empty() {
        Period::Overall
    } else {
        match args.rest() {
            "overall" | "all" => Period::Overall,
            "7day" | "7 day" | "7 days" | "week" | "1week" => Period::SevenDays,
            "1month" | "1months" | "1 month" | "1 months" | "month" => Period::OneMonth,
            "3month" | "3months" | "3 month" | "3 months" => Period::ThreeMonths,
            "6month" | "6months" | "6 month" | "6 months" => Period::SixMonths,
            "12month" | "12months" | "12 month" | "12 months" | "year" => Period::TwelveMonths,
            _ => {
                msg.reply(
                    ctx,
                    "Error: Invalid time period, valid options are `overall`, `7day`, `1month`, `3month`, `6month`, `12month`",
                )
                .await?;

                return Ok(());
            }
        }
    };

    let period_str = period.to_string();

    let lastfm_username = match UserData::from_id(ctx, msg.author.id)
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

    let typing = msg.channel_id.start_typing(&ctx.http)?;
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

    let mut top_artists = fm_client
        .top_artists(&lastfm_username)
        .await
        .within_period(period)
        .with_page(1)
        .with_limit(10)
        .send()
        .await
        .map(Arc::new)?;

    if top_artists.artists.is_empty() {
        msg.reply(ctx, "Error: No top artists were found").await?;

        return Ok(());
    };

    let mut top_artists_cache = Vec::new();
    top_artists_cache.push(Arc::clone(&top_artists));

    let mut page = 1;
    // API returns strings..
    let total_pages = top_artists.attrs.total_pages.parse::<usize>()?;

    let mut s = String::new();
    for artist in &top_artists.artists {
        let _ = writeln!(
            s,
            "{}) `{} plays` - [{}]({})",
            artist.attrs.rank, artist.scrobbles, artist.name, artist.url,
        );
    }

    let thumbnail_url = top_artists
        .artists
        .iter()
        .find(|artist| artist.images.get(2).is_some())
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
                    a.name(&top_artists.attrs.user);

                    a
                });
                e.title("Top Artists");
                e.description(s);

                e.thumbnail(thumbnail_url);
                e.footer(|f| {
                    f.text(format!("Page {}/{} • {}", page, total_pages, period_str));
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

                top_artists = if let Some(track) = top_artists_cache.get(page - 1).map(Arc::clone) {
                    track
                } else {
                    let new_top_artists = fm_client
                        .top_artists(&lastfm_username)
                        .await
                        .with_page(page)
                        .with_limit(10)
                        .send()
                        .await
                        .map(Arc::new)?;

                    top_artists_cache.push(Arc::clone(&new_top_artists));

                    new_top_artists
                };
            } else if r.emoji.unicode_eq("⬅️") {
                // Ignore on first page
                if page == 1 {
                    r.delete(&ctx).await?;
                    continue;
                }

                page -= 1;

                // Page is 1 indexed, Vec::get() is 0 indexed
                top_artists = top_artists_cache
                    .get(page - 1)
                    .map(Arc::clone)
                    .ok_or_else(|| Error::Sushii("Missing loved_tracks page in cache".into()))?;
            }

            let thumbnail_url = top_artists
                .artists
                .iter()
                .find(|artist| artist.images.get(2).is_some())
                .and_then(|t| t.images.get(2))
                .map_or_else(
                    || "https://i.imgur.com/oYm77EU.jpg",
                    |i| i.image_url.as_str(),
                );

            let mut s = String::new();
            for artist in &top_artists.artists {
                let _ = writeln!(
                    s,
                    "{}) `{} plays` - [{}]({})",
                    artist.attrs.rank, artist.scrobbles, artist.name, artist.url,
                );
            }

            sent_msg
                .edit(ctx, |m| {
                    m.embed(|e| {
                        e.colour(0xb90000);
                        e.author(|a| {
                            a.icon_url("https://i.imgur.com/C7u8gqg.jpg");
                            a.name(&top_artists.attrs.user);

                            a
                        });
                        e.title("Top Artists");
                        e.description(s);

                        e.thumbnail(thumbnail_url);
                        e.footer(|f| {
                            f.text(format!("Page {}/{} • {}", page, total_pages, period_str));
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
