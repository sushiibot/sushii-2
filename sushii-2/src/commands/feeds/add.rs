use futures::stream::StreamExt;
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{parse_channel, parse_role};
use std::fmt::Write;
use std::result::Result as StdResult;
use std::time::Duration;
use vlive::VLiveRequester;

use crate::error::Result;
use crate::keys::*;
use sushii_model::model::sql::{Feed, FeedMetadata, FeedSubscription};

#[derive(Default, Debug, Clone)]
struct FeedOptions {
    pub kind: Option<String>,
    pub discord_channel: Option<u64>,
    pub mention_role: Option<u64>,
}

impl FeedOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
pub trait UserOption<T> {
    fn display(&self, state: &T) -> String;
    fn prompt(&self) -> &'static str;
    async fn validate(
        &self,
        ctx: &Context,
        msg: &Message,
        state: &mut T,
    ) -> StdResult<String, String>;
}

pub struct OptionsCollector<T> {
    options: Vec<Box<dyn UserOption<T> + Send + Sync>>,
    /// Item to modify when an option is valid, where to store responses
    /// e.g a struct or hashmap
    state: T,
}

impl<T> OptionsCollector<T> {
    pub fn new(state: T) -> Self {
        Self {
            options: vec![],
            state,
        }
    }

    pub fn add_option(mut self, opt: impl UserOption<T> + 'static + Send + Sync) -> Self {
        self.options.push(Box::new(opt));
        self
    }

    pub fn get_state(&self) -> &T {
        &self.state
    }

    pub fn get_state_owned(self) -> T {
        self.state
    }

    async fn collect(
        &mut self,
        ctx: &Context,
        msg: &Message,
        sent_msg: Option<Message>,
        summary_str: Option<String>,
    ) -> Result<(Option<Message>, String, bool)> {
        let mut sent_msg: Option<Message> = sent_msg;
        let mut summary_str = summary_str.unwrap_or_else(String::new);

        'outer: for option in &self.options {
            let content = format!("{}~~-------------~~\n{}", summary_str, option.prompt());

            // Not first message, edit previous
            if let Some(ref mut sent_msg) = sent_msg {
                let embed = if let Some(e) = sent_msg.embeds.first() {
                    CreateEmbed::from(e.clone())
                } else {
                    let mut e = CreateEmbed::default();
                    e.title("Adding New Feed");
                    e.description(&content);
                    e.footer(|f| {
                        f.text("Type quit any time to exit");

                        f
                    });

                    e
                };

                sent_msg
                    .edit(ctx, |m| {
                        m.embed(|e| {
                            *e = embed;
                            e.description(&content);

                            e
                        });
                        m
                    })
                    .await?;
            } else {
                // First message
                sent_msg.replace(
                    msg.channel_id
                        .send_message(ctx, |m| {
                            m.embed(|e| {
                                e.title("Adding New Feed");
                                e.description(&content);
                                e.footer(|f| {
                                    f.text("Type quit any time to exit");

                                    f
                                });

                                e
                            })
                        })
                        .await?,
                );
            }

            let mut replies = msg
                .channel_id
                .await_replies(ctx)
                .author_id(msg.author.id)
                .channel_id(msg.channel_id)
                .timeout(Duration::from_secs(120))
                .await;

            'inner: while let Some(reply) = replies.next().await {
                match reply.content.as_str() {
                    "quit" | "exit" | "cancel" => {
                        msg.channel_id
                            .say(ctx, "Quitting, no feeds were added.")
                            .await?;

                        return Ok((None, "".into(), true));
                    }
                    _ => {}
                }

                match option.validate(ctx, &reply, &mut self.state).await {
                    Ok(_response) => {
                        // Add option description to summary
                        writeln!(summary_str, "{}", option.display(&self.state))?;
                        reply.delete(ctx).await?;
                        // msg.channel_id.say(ctx, response).await?;

                        // If success, break from waiting for response and go to
                        // next option
                        continue 'outer;
                    }
                    Err(response) => {
                        // If option isn't valid, respond with error and wait for another try
                        msg.channel_id
                            .say(ctx, format!("Error: {}", response))
                            .await?;

                        // Get another reply
                        continue 'inner;
                    }
                }
            }

            msg.channel_id
                .say(ctx, "Timed out (2 min), no feeds were added.")
                .await?;

            return Ok((None, "".into(), true));
            // TODO: Delete messages
        }

        Ok((sent_msg, summary_str, false))
    }
}

struct FeedType;

#[async_trait]
impl UserOption<FeedOptions> for FeedType {
    fn display(&self, state: &FeedOptions) -> String {
        if let Some(kind) = &state.kind {
            format!("Feed Type: {}", kind)
        } else {
            "Feed Type: ".into()
        }
    }

    fn prompt(&self) -> &'static str {
        "What kind of feed do you want to add? Currently available feeds are: `vlive`"
    }

    async fn validate(
        &self,
        _ctx: &Context,
        msg: &Message,
        state: &mut FeedOptions,
    ) -> StdResult<String, String> {
        match msg.content.as_str() {
            "vlive" => {
                state.kind.replace("vlive".into());
                return Ok(format!("Selected feed type {}", msg.content));
            }
            // RSS feeds later
            // "twitter" | "youtube" => "rss",
            _ => {
                return Err(format!(
                    "{} is not a valid feed type. Currently available feeds are: `vlive`",
                    msg.content
                ));
            }
        }
    }
}

struct DiscordChannel;

#[async_trait]
impl UserOption<FeedOptions> for DiscordChannel {
    fn display(&self, state: &FeedOptions) -> String {
        if let Some(channel) = state.discord_channel {
            format!("Discord Channel: <#{}>", channel)
        } else {
            "Discord Channel: ".into()
        }
    }

    fn prompt(&self) -> &'static str {
        "Which channel do you want updates to be sent to?"
    }

    async fn validate(
        &self,
        ctx: &Context,
        msg: &Message,
        state: &mut FeedOptions,
    ) -> StdResult<String, String> {
        let channel_id = msg
            .content
            .parse::<u64>()
            .ok()
            .or_else(|| parse_channel(&msg.content))
            .ok_or_else(|| "Invalid channel. Give a channel.".to_string())?;

        let guild_channels = msg
            .guild_field(ctx, |g| g.channels.clone())
            .await
            .ok_or_else(|| "Couldn't find channel. Give a channel.".to_string())?;

        match guild_channels.get(&ChannelId(channel_id)) {
            Some(c) => {
                if c.kind != ChannelType::Text {
                    return Err("Channel is not a text channel. Try a different one.".into());
                }

                state.discord_channel.replace(c.id.0);

                return Ok(format!("Updates will be sent to <#{}>", c.id.0));
            }
            None => {
                return Err("Channel is not found in this guild. Try again?".into());
            }
        }
    }
}

struct DiscordRole;

#[async_trait]
impl UserOption<FeedOptions> for DiscordRole {
    fn display(&self, state: &FeedOptions) -> String {
        if let Some(mention_role) = state.mention_role {
            format!("Mention Role: <@&{}>", mention_role)
        } else {
            "Mention Role: None".into()
        }
    }

    fn prompt(&self) -> &'static str {
        "What role do you want to mention for new updates? Say `none` for no role mention."
    }

    async fn validate(
        &self,
        ctx: &Context,
        msg: &Message,
        state: &mut FeedOptions,
    ) -> StdResult<String, String> {
        if msg.content.trim() == "none" {
            return Ok("This feed won't mention any roles.".into());
        }

        // TODO: actually handle this, not an accurate error
        let guild_roles = msg
            .guild_field(ctx, |g| g.roles.clone())
            .await
            .ok_or_else(|| "Couldn't find role. Give a role.".to_string())?;

        let mention_role = parse_role(&msg.content)
            .or_else(|| msg.content.parse::<u64>().ok())
            .or_else(|| {
                guild_roles
                    .values()
                    .find(|&x| x.name.to_lowercase() == msg.content.to_lowercase())
                    .map(|x| x.id.0)
            })
            .ok_or_else(|| "Invalid role, give a role name, role mention, or role ID. `none` for no mention role.".to_string())?;

        state.mention_role.replace(mention_role);

        Ok(format!("The role will be mentioned <@&{}>", mention_role))
    }
}

#[command]
#[only_in("guild")]
#[required_permissions("MANAGE_GUILD")]
async fn add(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
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

    let mut options_collector = OptionsCollector::new(FeedOptions::new())
        .add_option(FeedType)
        .add_option(DiscordChannel)
        .add_option(DiscordRole);

    let (sent_msg, summary_str, should_quit) =
        options_collector.collect(ctx, msg, None, None).await?;
    if should_quit {
        return Ok(());
    }

    let opts = options_collector.get_state();

    let feed_metadata = match opts.kind.as_ref().map(|k| k.to_lowercase()).as_deref() {
        Some("vlive") => match add_vlive(reqwest, ctx, msg, sent_msg, summary_str).await? {
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

    // Need to save the Feed data, or ensure it already exists before saving subscription
    let feed = Feed::from_meta(feed_metadata).save(ctx).await?;

    // TODO: Check if subscription already exists

    let subscription = FeedSubscription::new(
        feed.feed_id.clone(),
        guild.id.0 as i64,
        opts.discord_channel.unwrap() as i64,
    )
    .mention_role(opts.mention_role.map(|r| r as i64))
    .save(ctx)
    .await?;

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Added new feed");
                e.author(|a| {
                    if let Some(url) = feed.icon_url() {
                        a.icon_url(url);
                    }
                    a.name(feed.name().unwrap_or("Unknown Feed"));
                    if let Some(url) = feed.source_url() {
                        a.url(url);
                    }

                    a
                });
                e.field(
                    "Discord Channel",
                    format!("<#{}>", subscription.channel_id as u64),
                    true,
                );

                if let Some(id) = subscription.mention_role {
                    e.field("Mention Role", format!("<@&{}>", id as u64), true);
                } else {
                    e.field("Mention Role", "No role", true);
                }

                e
            });

            m
        })
        .await?;

    Ok(())
}

#[derive(Default, Debug, Clone)]
struct VliveOptions {
    pub feed_metadata: Option<FeedMetadata>,
}

impl VliveOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

struct VliveChannelStep;

#[async_trait]
impl UserOption<VliveOptions> for VliveChannelStep {
    fn display(&self, state: &VliveOptions) -> String {
        if let Some(feed_metadata) = &state.feed_metadata {
            format!("Vlive Channel: {}", feed_metadata.name())
        } else {
            "Vlive Channel: ".into()
        }
    }

    fn prompt(&self) -> &'static str {
        "What vlive channel? Give a vlive channel code. You can find this \
        in the channel URL, for example `F001E5` is the code for \
        `https://www.vlive.tv/channel/F001E5`"
    }

    async fn validate(
        &self,
        ctx: &Context,
        msg: &Message,
        state: &mut VliveOptions,
    ) -> StdResult<String, String> {
        let reqwest = ctx
            .data
            .read()
            .await
            .get::<ReqwestContainer>()
            .cloned()
            .unwrap();

        let channel = reqwest.get_channel_info(&msg.content).await.map_err(|_| {
            format!(
                "No channel was found with code `{}`. \
                Give a vlive channel code. You can find this \
                in the channel URL, for example `F001E5` is the code for \
                `https://www.vlive.tv/channel/F001E5`.",
                &msg.content
            )
        })?;

        state.feed_metadata.replace(FeedMetadata::vlive_videos(
            None,
            channel.channel_code,
            channel.channel_name.clone(),
            channel.channel_profile_image,
        ));

        Ok(format!("Found channel {}", &channel.channel_name))
    }
}

// vlive feeds are hardcoded since needs to search channels, handle more stuff etc
async fn add_vlive(
    _reqwest: reqwest::Client,
    ctx: &Context,
    msg: &Message,
    sent_msg: Option<Message>,
    summary_str: String,
) -> Result<Option<FeedMetadata>> {
    let _messages = msg
        .channel_id
        .await_replies(ctx)
        .author_id(msg.author.id)
        .channel_id(msg.channel_id)
        .await;

    let mut options_collector =
        OptionsCollector::new(VliveOptions::new()).add_option(VliveChannelStep);

    let (sent_msg, _, should_quit) = options_collector
        .collect(ctx, msg, sent_msg, Some(summary_str))
        .await?;

    if should_quit {
        return Ok(None);
    }

    if let Some(sent_msg) = sent_msg {
        sent_msg.delete(ctx).await?;
    }

    let opts = options_collector.get_state_owned();

    Ok(opts.feed_metadata)
}
