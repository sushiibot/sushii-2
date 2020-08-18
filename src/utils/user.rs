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
