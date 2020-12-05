use serenity::collector::reaction_collector::ReactionAction;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;
use std::time::Duration;
use tokio::stream::StreamExt;

use crate::model::sql::*;
use crate::model::Paginator;

const PAGE_SIZE: i64 = 10;

fn fmt_tags(tags: &Vec<Tag>, highlight: &str) -> String {
    let mut s = String::new();

    for tag in tags {
        let tag_name_highlighted = tag
            .tag_name
            .replace(highlight, &format!("__{}__", highlight));
        let _ = writeln!(s, "{}", tag_name_highlighted);
    }

    s
}

#[command]
async fn search(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx, "Error: Not in guild").await?;
            return Ok(());
        }
    };

    let query = match args.single::<String>() {
        Ok(q) => q,
        Err(_) => {
            msg.channel_id
                .say(&ctx, "Error: Please give a search")
                .await?;

            return Ok(());
        }
    };

    let tag_count = Tag::get_search_count(&ctx, guild_id, &query).await?;

    if tag_count == 0 {
        msg.channel_id
            .say(
                &ctx,
                format!("Error: There are no tags found containing {}", query),
            )
            .await?;

        return Ok(());
    }

    // Page size 10
    let mut paginator = Paginator::new(PAGE_SIZE, tag_count);
    let mut tags = Tag::search(&ctx, guild_id, &query, PAGE_SIZE, None).await?;

    let mut sent_msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!(
                    "Server tags containing {} ({} total)",
                    query, tag_count
                ));
                e.description(fmt_tags(&tags, &query));
                e.footer(|f| {
                    f.text(format!(
                        "Page {}/{}",
                        paginator.current_page, paginator.page_count
                    ));
                    f
                });

                e
            });

            // Only add reactions if theres multiple pages
            if paginator.page_count > 1 {
                m.reactions(vec![
                    ReactionType::Unicode("⬅️".into()),
                    ReactionType::Unicode("➡️".into()),
                ]);
            }

            m
        })
        .await?;

    // Don't listen for reactions if theres only 1 page
    if paginator.page_count <= 1 {
        return Ok(());
    }

    while let Some(reaction_action) = sent_msg
        .await_reactions(&ctx)
        .author_id(msg.author.id)
        .filter(|r| ["⬅️", "➡️"].iter().any(|u| r.emoji.unicode_eq(u)))
        .timeout(Duration::from_secs(45))
        .await
        .next()
        .await
    {
        match *reaction_action {
            ReactionAction::Added(ref r) => {
                // tracing::info!("offsets: {:?}", offsets);

                // Next page
                if r.emoji.unicode_eq("➡️") {
                    let offset = tags.last().map(|t| t.tag_name.clone());
                    if !paginator.next(offset.clone()) {
                        r.delete(&ctx).await?;
                        continue;
                    }

                    // Get next page
                    tags =
                        Tag::search(&ctx, guild_id, &query, PAGE_SIZE, offset.as_deref()).await?;
                } else if r.emoji.unicode_eq("⬅️") {
                    // Ignore on first page
                    if paginator.current_page == 1 {
                        r.delete(&ctx).await?;
                        continue;
                    }

                    // Use previous page's last tag as offset
                    tags = Tag::search(
                        &ctx,
                        guild_id,
                        &query,
                        PAGE_SIZE,
                        paginator.prev_offset().map(|o| o.as_str()),
                    )
                    .await?;
                }

                sent_msg
                    .edit(&ctx, |m| {
                        m.embed(|e| {
                            e.title(format!("Server containing {} ({} total)", query, tag_count));
                            e.description(fmt_tags(&tags, &query));
                            e.footer(|f| {
                                f.text(format!(
                                    "Page {}/{}",
                                    paginator.current_page, paginator.page_count
                                ));
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
            _ => {}
        }
    }

    // Delete all reactions after timed out to show user they can't react anymore
    sent_msg.delete_reactions(&ctx).await?;

    Ok(())
}
