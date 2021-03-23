use chrono::Utc;
use heim::cpu::os::unix;
use heim::units::{information, ratio, time};
use heim::{memory, process};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::shard_id;
use std::time::Duration;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[description("Gets the invite link for sushii")]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO: Pass invite link via config
    msg.channel_id.say(
        ctx,
        "https://discord.com/oauth2/authorize?client_id=193163942502072320&permissions=268823622&scope=applications.commands%20bot"
    ).await?;

    Ok(())
}

#[command]
#[aliases("stats")]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let version = env!("CARGO_PKG_VERSION");

    // Host CPU
    let (one, five, fifteen) = unix::loadavg().await?;

    let proc = process::current().await?;
    let start_time = proc.create_time().await?;

    let now = Utc::now().timestamp();
    let up_time = Duration::from_secs_f64(now as f64 - start_time.get::<time::second>().floor());

    // Process memory
    let mem = proc.memory().await?.rss();
    // Host total memory
    let total_mem = memory::memory().await?.total();

    // Discord stats
    let guild_count = ctx.cache.guild_count().await;
    let channel_count = ctx.cache.guild_channel_count().await;
    let user_count = ctx.cache.user_count().await;

    let shard_count = ctx.cache.shard_count().await;
    let shard_id = msg.guild_id.map(|id| shard_id(id, shard_count));

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!(
                    "sushii v{} - {}",
                    version,
                    env!("VERGEN_BUILD_TIMESTAMP")
                ));
                e.color(0xe67e22);

                e.field("Servers", guild_count.to_string(), true);
                e.field("Shards", shard_count.to_string(), true);
                e.field("Channels", channel_count.to_string(), true);
                e.field("Cached Users", user_count.to_string(), true);

                e.field(
                    "Load Avg",
                    format!(
                        "{}, {}, {}",
                        one.get::<ratio::ratio>(),
                        five.get::<ratio::ratio>(),
                        fifteen.get::<ratio::ratio>(),
                    ),
                    true,
                );
                e.field(
                    "Memory Usage",
                    format!(
                        "{} / {} MB",
                        mem.get::<information::megabyte>(),
                        total_mem.get::<information::megabyte>()
                    ),
                    true,
                );
                e.field("Uptime", humantime::format_duration(up_time), false);
                e.field(
                    "Rust Version",
                    format!(
                        "{} - {}",
                        env!("VERGEN_RUSTC_SEMVER"),
                        env!("VERGEN_RUSTC_COMMIT_DATE"),
                    ),
                    true,
                );

                if let Some(shard_id) = shard_id {
                    e.footer(|f| f.text(format!("Shard #{}", shard_id.to_string())));
                }

                e
            })
        })
        .await?;

    Ok(())
}
