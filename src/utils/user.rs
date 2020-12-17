use serenity::utils::parse_mention;
use serenity::{model::prelude::*, prelude::Context};

pub async fn get_user(ctx: &Context, id: u64) -> Option<User> {
    // First check cache then try http
    let cached_user = ctx.cache.user(id as u64).await;

    if cached_user.is_some() {
        return cached_user;
    }

    // Try fetching via http
    ctx.http.get_user(id).await.ok()
}

// Parses a string to ID, either a raw ID or a mention
pub fn parse_id<S: AsRef<str>>(s: S) -> Option<u64> {
    parse_mention(s.as_ref()).or_else(|| s.as_ref().parse::<u64>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ids() {
        let strs = vec!["249202547565264896", "<@249202547565264896>"];
        let exp = Some(249202547565264896);

        for s in strs {
            let id = parse_id(s);

            assert_eq!(id, exp);
        }
    }
}
