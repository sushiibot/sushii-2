use crate::error::Result;

pub struct SushiiConfig {
    pub discord_token: String,
    pub owner_ids: Vec<u64>,
    pub database_url: String,
    pub default_prefix: String,
    pub blocked_users: Vec<u64>,
    pub lastfm_key: String,
}

fn parse_id_array(s: &str) -> Vec<u64> {
    s.split(",")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|u| u.parse::<u64>().ok())
        .collect()
}

impl SushiiConfig {
    pub fn new_from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(SushiiConfig {
            discord_token: dotenv::var("DISCORD_TOKEN")?,
            owner_ids: parse_id_array(&dotenv::var("OWNER_IDS").unwrap_or("".into())),
            database_url: dotenv::var("DATABASE_URL")?,
            default_prefix: dotenv::var("DEFAULT_PREFIX")?,
            blocked_users: parse_id_array(&dotenv::var("BLOCKED_USERS").unwrap_or("".into())),
            lastfm_key: dotenv::var("LASTFM_KEY").unwrap_or("".into()),
        })
    }
}
