use serenity::collector::reaction_collector::ReactionAction;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fmt::Write;
use std::time::Duration;
use tokio_stream::StreamExt;

use crate::model::sql::*;
use crate::model::Paginator;
use crate::utils::user::parse_id;

const PAGE_SIZE: i64 = 20;

fn fmt_tags(tags: &[Tag]) -> String {
    let mut s = String::new();

    for tag in tags {
        let _ = writeln!(s, "{}", tag.tag_name);
    }

    s
}

#[command]
async fn author(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx, "Error: Not in guild").await?;
            return Ok(());
        }
    };

    let target_str = args.rest();

    let user_id = match parse_id(target_str) {
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

    let target_user = match user_id.to_user(&ctx).await {
        Ok(u) => u,
        Err(_) => {
            msg.reply(
                &ctx,
                "Error: Failed to fetch user, are you using a correct user ID? (Could be a message ID)",
            )
            .await?;

            return Ok(());
        }
    };

    let tag_count = Tag::get_all_author_count(&ctx, guild_id, target_user.id).await?;

    if tag_count == 0 {
        msg.channel_id
            .say(
                &ctx,
                format!("Error: There are no tags created by {}", target_user.tag()),
            )
            .await?;

        return Ok(());
    }

    // Page size 10
    let mut paginator = Paginator::new(PAGE_SIZE, tag_count);
    let mut tags = Tag::get_all_author(&ctx, guild_id, target_user.id, PAGE_SIZE, None).await?;

    let mut sent_msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!(
                    "Server tags created by {} ({} total)",
                    target_user.tag(),
                    tag_count
                ));
                e.description(fmt_tags(&tags));
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
        if let ReactionAction::Added(ref r) = *reaction_action {
            // tracing::info!("offsets: {:?}", offsets);

            // Next page
            if r.emoji.unicode_eq("➡️") {
                let offset = tags.last().map(|t| t.tag_name.clone());
                if !paginator.next(offset.clone()) {
                    r.delete(&ctx).await?;
                    continue;
                }

                // Get next page
                tags = Tag::get_all_author(
                    &ctx,
                    guild_id,
                    target_user.id,
                    PAGE_SIZE,
                    offset.as_deref(),
                )
                .await?;
            } else if r.emoji.unicode_eq("⬅️") {
                // Ignore on first page
                if paginator.current_page == 1 {
                    r.delete(&ctx).await?;
                    continue;
                }

                // Use previous page's last tag as offset
                tags = Tag::get_all_author(
                    &ctx,
                    guild_id,
                    target_user.id,
                    PAGE_SIZE,
                    paginator.prev_offset().map(|o| o.as_str()),
                )
                .await?;
            }

            sent_msg
                .edit(&ctx, |m| {
                    m.embed(|e| {
                        e.title(format!(
                            "Server tags created by {} ({} total)",
                            target_user.tag(),
                            tag_count
                        ));
                        e.description(fmt_tags(&tags));
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
    }

    // Delete all reactions after timed out to show user they can't react anymore
    sent_msg.delete_reactions(&ctx).await?;

    Ok(())
}
