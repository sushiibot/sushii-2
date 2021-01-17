use futures::stream::StreamExt;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{parse_channel, parse_role};
use vlive::VLiveRequester;

use crate::error::Result;
use crate::keys::*;
use sushii_model::model::sql::{FeedMetadata, FeedSubscription};
use sushii_model::model::sql::feed::feed::Id;

enum BasicState {
    FeedType,
    DiscordChannel,
    DiscordRole,
}

impl BasicState {
    pub fn start() -> Self {
        Self::FeedType
    }

    pub fn prompt(&self) -> &'static str {
        match self {
            Self::FeedType => "What kind of feed do you want to add? Currently available feeds are: `vlive`, `twitter`",
            Self::DiscordChannel => "Which channel do you want updates to be sent to?",
            Self::DiscordRole => "What role do you want to mention for new updates? Say `none` for no role mention.",
        }
    }

    pub fn next(self) -> Option<Self> {
        match self {
            Self::FeedType => Some(Self::DiscordChannel),
            Self::DiscordChannel => Some(Self::DiscordRole),
            Self::DiscordRole => None,
        }
    }
}

#[command]
#[only_in("guild")]
#[required_permissions("MANAGE_GUILD")]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = match msg.guild(ctx).await {
        Some(g) => g,
        None => {
            msg.reply(&ctx, "Error: No guild").await?;

            return Ok(());
        }
    };

    let reqwest = ctx
        .data
        .read()
        .await
        .get::<ReqwestContainer>()
        .cloned()
        .unwrap();

    let mut state = BasicState::start();

    let mut messages = msg
        .channel_id
        .await_replies(ctx)
        .author_id(msg.author.id)
        .channel_id(msg.channel_id)
        .await;

    msg.channel_id.say(&ctx, state.prompt()).await?;

    let mut feed_type = "";
    let mut target_channel = 0;
    let mut mention_role = None;

    while let Some(reply) = messages.next().await {
        let mut reply_str = String::new();

        match state {
            BasicState::FeedType => {
                match reply.content.as_str() {
                    "vlive" => {
                        reply_str = format!("Selected feed type {}", reply.content);
                        feed_type = "vlive";
                    }
                    // RSS feeds later
                    // "twitter" | "youtube" => "rss",
                    _ => {
                        msg.channel_id
                            .say(
                            &ctx,
                            format!("{} is not a valid feed type. Currently available feeds are: `vlive`", msg.content),
                        )
                        .await?;

                        // Continue if error
                        continue;
                    }
                }
            }
            BasicState::DiscordChannel => {
                let channel_id = match reply
                    .content
                    .parse::<u64>()
                    .ok()
                    .or_else(|| parse_channel(&reply.content))
                {
                    Some(c) => c,
                    None => {
                        msg.channel_id
                            .say(&ctx, "Error: Invalid channel. Give a channel.")
                            .await?;

                        continue;
                    }
                };

                target_channel = match guild.channels.get(&ChannelId(channel_id)) {
                    Some(c) => {
                        if c.kind != ChannelType::Text {
                            msg.channel_id
                                .say(
                                    &ctx,
                                    "Error: Channel is not a text channel. Try a different one.",
                                )
                                .await?;

                            continue;
                        }

                        reply_str = format!("Updates will be sent to <#{}>", c.id.0);

                        c.id.0
                    }
                    None => {
                        msg.channel_id
                            .say(
                                &ctx,
                                "Error: Channel is not found in this guild. Try again?",
                            )
                            .await?;

                        // Continue if error
                        continue;
                    }
                };
            }
            BasicState::DiscordRole => {
                if reply.content.trim() == "none" {
                    reply_str = "This feed won't mention any roles.".into();
                } else {
                    mention_role = match parse_role(&reply.content)
                        .or_else(|| reply.content.parse::<u64>().ok())
                        .or_else(|| {
                            guild
                                .roles
                                .values()
                                .find(|&x| x.name.to_lowercase() == reply.content.to_lowercase())
                                .map(|x| x.id.0)
                        }) {
                        Some(r) => {
                            reply_str = format!("The role will be mentioned <@&{}>", r);

                            Some(r)
                        }
                        None => {
                            msg.channel_id
                                    .say(
                                    &ctx,
                                    "Error: Invalid role, give a role name, role mention, or role ID. `none` for no mention role.",
                                )
                                .await?;

                            continue;
                        }
                    };
                }
            }
        }

        state = match state.next() {
            Some(s) => {
                // Respond with reply and prompt for next question
                msg.channel_id
                    .say(ctx, format!("{}\n{}", reply_str, s.prompt()))
                    .await?;

                s
            }
            None => {
                // Respond with reply without next prompt
                msg.channel_id.say(ctx, reply_str).await?;

                break;
            }
        };
    }

    let feed_metadata = match feed_type.to_lowercase().as_str() {
        "vlive" => match add_vlive(reqwest, ctx, msg).await? {
            Some(m) => m,
            None => return Ok(()),
        },
        // "twitter" | "twt" => {}
        _ => {
            msg.reply(
                &ctx,
                "Error: Invalid feed type. \
                    Currently available feeds are: `vlive`",
            )
            .await?;

            return Ok(());
        }
    };

    let subscription =
        FeedSubscription::new(feed_metadata.id(), guild.id.0 as i64, target_channel as i64)
            .mention_role(mention_role.map(|r| r as i64));

    dbg!(subscription);

    Ok(())
}

enum VliveStep {
    Channel,
    Board,
}

impl VliveStep {
    pub fn start() -> Self {
        Self::Channel
    }

    pub fn next(self, is_videos: bool) -> Option<Self> {
        match self {
            Self::Channel if is_videos => None,
            Self::Channel => Some(Self::Board),
            Self::Board => None,
        }
    }
}

// vlive feeds are hardcoded since needs to search channels, handle more stuff etc
async fn add_vlive(
    reqwest: reqwest::Client,
    ctx: &Context,
    msg: &Message,
) -> Result<Option<FeedMetadata>> {
    let mut messages = msg
        .channel_id
        .await_replies(ctx)
        .author_id(msg.author.id)
        .channel_id(msg.channel_id)
        .await;

    let mut step = VliveStep::start();

    msg.channel_id
        .say(
            &ctx,
            "What vlive channel? Give a vlive channel code. You can find this \
            in the channel URL, for example `F001E5` is the code for \
            `https://www.vlive.tv/channel/F001E5` Type `quit` any time to stop.",
        )
        .await?;

    let mut feed_metadata = None;

    while let Some(reply) = messages.next().await {
        if reply.content == "quit" {
            msg.channel_id
                .say(&ctx, "Quitting. No feeds were added.")
                .await?;

            return Ok(None);
        }

        match step {
            VliveStep::Channel => {
                let channel = match reqwest.get_channel_info(&reply.content).await {
                    Ok(c) => c,
                    Err(_) => {
                        msg.reply(
                            &ctx,
                            format!(
                                "Error: No channel was found with code `{}`. \
                                Give a vlive channel code. You can find this \
                                in the channel URL, for example `F001E5` is the code for \
                                `https://www.vlive.tv/channel/F001E5`. Type `quit` any time to stop.",
                                &reply.content
                            ),
                        )
                        .await?;

                        continue;
                    }
                };

                msg.reply(&ctx, format!("Found channel {}", &channel.name))
                    .await?;

                feed_metadata = Some(FeedMetadata::vlive_videos(
                    None,
                    channel.channel_code,
                    channel.name,
                    channel.profile_img,
                ));
            }
            VliveStep::Board => {
                break;
            }
        }

        step = match step.next(true) {
            Some(s) => s,
            None => break,
        };
    }

    Ok(feed_metadata)
}
