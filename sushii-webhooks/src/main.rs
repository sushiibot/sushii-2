use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use twilight_http::Client;
use twilight_model::channel::message::allowed_mentions::AllowedMentions;
use twilight_model::id::UserId;
use warp::http::StatusCode;
use warp::Filter;

mod error;
use error::Error;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub top_gg_auth: String,
    pub twilight_api_proxy_url: String,
    pub public_log_channel: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();

        cfg.set_default("listen_addr", "0.0.0.0:8080")?;
        cfg.merge(config::Environment::new())?;
        Ok(cfg.try_into()?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TopGgBotVote {
    /// Discord ID of the bot that received a vote.
    pub bot: String,
    /// Discord ID of the user who voted.
    pub user: String,
    /// The type of the vote (should always be "upvote" except when using the test button it's "test").
    #[serde(rename = "type")]
    pub kind: String,
    /// Whether the weekend multiplier is in effect, meaning users votes count as two.
    pub is_weekend: bool,
    /// Query string params found on the /bot/:ID/vote page. Example: ?a=1&b=2.
    pub query: Option<String>,
}

fn with_twilight_http(
    http: Client,
) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || http.clone())
}

fn with_config(
    config: Config,
) -> impl Filter<Extract = (Config,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}

async fn _send_messages(user_id: u64, http: Client, config: Config) -> Result<(), Error> {
    let channel = http.create_private_channel(UserId(user_id)).await?;
    http.create_message(channel.id)
        .content("Thanks for voting on top.gg!")?
        .await?;

    http.create_message(config.public_log_channel.into())
        .content(format!(
            "<@{}> voted on top.gg! <a:sushiiSpin:828894443309367306>",
            user_id
        ))?
        .allowed_mentions(AllowedMentions::builder().build())
        .await?;

    Ok(())
}

async fn send_messages(
    vote: TopGgBotVote,
    http: Client,
    config: Config,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = vote
        .user
        .parse::<u64>()
        .map_err(|_| warp::reject::custom(Error::InvalidId))?;

    // Don't want this to block response, if DM fails then that's fine whatever too
    tokio::spawn(async move {
        if let Err(e) = _send_messages(user_id, http, config).await {
            tracing::warn!(user_id, "Failed to send messages: {}", e);
        }
    });

    // Immediately respond with 204
    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let cfg = Config::from_env().expect("Failed to create config");

    let http = Client::builder()
        .proxy(cfg.twilight_api_proxy_url.clone(), true)
        .ratelimiter(None)
        .build();

    // Warp stuff
    let logger = warp::log("sushii_webhooks");
    // https://github.com/seanmonstar/warp/issues/503
    let top_gg_auth = warp::header::exact(
        "Authorization",
        Box::leak(Box::new(cfg.top_gg_auth.clone())),
    );

    // POST /webhook/topgg
    let topgg_webhook = warp::post()
        .and(warp::path!("webhook" / "topgg"))
        .and(top_gg_auth)
        // Only accept bodies smaller than 16kb
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(with_twilight_http(http))
        .and(with_config(cfg.clone()))
        .and_then(send_messages)
        .with(logger);

    tracing::info!("Listening on {}", cfg.listen_addr);

    warp::serve(topgg_webhook).run(cfg.listen_addr).await;
}
