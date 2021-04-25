use serde::{Deserialize, Serialize};
use warp::Filter;
use warp::http::StatusCode;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let logger = warp::log("sushii_webhooks");

    // POST /webhook/topgg
    let topgg_webhook = warp::post()
        .and(warp::path!("webhook" / "topgg"))
        // Only accept bodies smaller than 16kb
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::form())
        .map(|vote: TopGgBotVote| {
            tracing::info!("Vote received: {:?}", vote);
            Ok(StatusCode::NO_CONTENT)
        })
        .with(logger);

    warp::serve(topgg_webhook)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
