use chrono::Utc;
use heim::cpu::os::unix;
use heim::units::{information, ratio, time};
use heim::{memory, process};
use serenity::client::bridge::gateway::ShardId;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::shard_id;
use sqlx::Connection;
use std::time::{Duration, Instant};
use sushii_model::prelude::DbPool;

use crate::keys::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let now = Instant::now();
    let mut sent_msg = msg.channel_id.say(&ctx.http, "Ping!").await?;
    let msg_ms = now.elapsed().as_millis();

    let pool = ctx.data.read().await.get::<DbPool>().cloned().unwrap();
    let mut conn = pool.acquire().await?;

    let now = Instant::now();
    conn.ping().await?;
    let pg_ms = now.elapsed().as_micros();

    let shard_manager = ctx
        .data
        .read()
        .await
        .get::<ShardManagerContainer>()
        .cloned()
        .unwrap();

    let shard_latency_ms = shard_manager
        .lock()
        .await
        .runners
        .lock()
        .await
        .get(&ShardId(ctx.shard_id))
        .and_then(|s| s.latency);

    let shard_latency_ms_str = shard_latency_ms
        .map(|d| {
            format!(
                "{:.3}",
                d.as_secs() as f64 / 1000.0 + f64::from(d.subsec_nanos()) * 1e-6
            )
        })
        .unwrap_or_else(|| "N/A".into());

    sent_msg
        .edit(ctx, |m| {
            m.content("");

            m.embed(|e| {
                e.title("Pong!");
                e.description(format!(
                    "Discord Rest API (message send): `{} ms`\n\
                    Discord Shard latency (heartbeat ACK): `{} ms`\n\
                    PostgreSQL latency: `{} μs`\n",
                    msg_ms, shard_latency_ms_str, pg_ms,
                ))
            })
        })
        .await?;

    Ok(())
}

#[command]
#[description("Gets the invite link for sushii")]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO: Pass invite link via config
    msg.channel_id.say(
        ctx,
        "You can invite sushii with this link! \n\
        <https://discord.com/oauth2/authorize?client_id=193163942502072320&permissions=268823622&scope=applications.commands%20bot>"
    ).await?;

    Ok(())
}

#[command]
#[aliases("donate", "support")]
async fn patreon(ctx: &Context, msg: &Message) -> CommandResult {
    // TODO: Pass invite link via config
    msg.channel_id
        .say(
            ctx,
            "You can support me here! <https://www.patreon.com/sushiibot>",
        )
        .await?;

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
