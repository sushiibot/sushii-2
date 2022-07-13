use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;

use crate::model::moderation::{ModActionExecutor, ModActionType};
use sushii_model::model::sql::GuildConfig;

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    ModActionExecutor::from_args(args, ModActionType::Ban)
        .execute(&ctx, &msg, &guild_id)
        .await?;

    // Prompt user for lookup optin

    let mut guild_config = match GuildConfig::from_msg(ctx, &msg).await? {
        Some(c) => c,
        None => return Ok(()),
    };

    // If guild already opted in directly with optin command
    if guild_config.data.lookup_prompted || !guild_config.data.lookup_details_opt_in {
        return Ok(());
    }

    let mut opt_in_msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Lookup Opt-In");
                e.description("Hi! There's a new command `lookup` to look up a user's bans across servers sushii is in. \n\
                               **By default, your server and other servers will show as anonymous without the server name or ban reason.** \n\n\
                               If you want to see other server names for looked up users, you can opt-in to sharing your server name and ban reasons. \
                               You can opt out at any time if you later choose not to share your server name and ban reasons with `lookup optout`.");

                e
            });

            m.components(|comps| {
                comps.create_action_row(|action_row| {
                    action_row.create_button(|button| {
                        button
                            .label("Opt in")
                            .style(ButtonStyle::Primary)
                            .custom_id("opt_in")
                    });

                    action_row.create_button(|button| {
                        button
                            .label("Stay opted out")
                            .style(ButtonStyle::Secondary)
                            .custom_id("opt_out")
                    });

                    action_row
                });
                comps
            });

            m
        })
        .await?;

    if let Some(component_interaction) = opt_in_msg
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(120))
        .author_id(msg.author.id)
        .await
    {
        match component_interaction.data.custom_id.as_str() {
            "opt_in" => {
                component_interaction
                    .create_interaction_response(&ctx.http, |res| {
                        res.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await?;

                guild_config.data.lookup_details_opt_in = true;
                guild_config.data.lookup_prompted = true;
                guild_config.save(ctx).await?;

                opt_in_msg
                        .edit(&ctx.http, move |msg| {
                            msg.embed(|e| {
                                e.title("Opted In!");
                                e.description("You are now sharing server name and ban reasons, \
                                               and you are able to view other server name and reasons for those who have opted in. \
                                               You can opt-out with `lookup optout`.")
                            });

                            msg.components(|comps| {
                                comps.set_action_rows(Vec::new());
                                comps
                            });
                            msg
                        })
                        .await?;

                return Ok(());
            }
            "opt_out" => {
                component_interaction
                    .create_interaction_response(&ctx.http, |res| {
                        res.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await?;

                guild_config.data.lookup_prompted = true;
                guild_config.save(ctx).await?;

                opt_in_msg
                        .edit(&ctx.http, move |msg| {
                            msg.embed(|e| {
                                e.title("Opted Out!");
                                e.description("This server is listed as anonymous \
                                               and you won't be able to view other server name and reasons. \
                                               You can opt-in any time with `lookup optin`. \
                                               This prompt won't show again.")
                            });

                            msg.components(|comps| {
                                comps.set_action_rows(Vec::new());
                                comps
                            });
                            msg
                        })
                        .await?;

                return Ok(());
            }
            _ => {
                tracing::error!("Unhandled reason interaction: {:?}", component_interaction);
                return Ok(());
            }
        }
    }

    opt_in_msg
        .edit(&ctx.http, move |msg| {
            msg.components(|comps| {
                comps.set_action_rows(Vec::new());
                comps
            });
            msg
        })
        .await?;

    Ok(())
}

#[command]
#[only_in("guild")]
#[required_permissions("BAN_MEMBERS")]
async fn unban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "No guild found").await?;

            return Ok(());
        }
    };

    ModActionExecutor::from_args(args, ModActionType::Unban)
        .execute(&ctx, &msg, &guild_id)
        .await?;

    Ok(())
}
