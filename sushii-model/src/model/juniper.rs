use crate::model::sql::user::cached_user::{CachedUserBatcher, CachedUserLoader};
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub pool: Arc<sqlx::PgPool>,
    pub cached_user_loader: CachedUserLoader,
}

impl Context {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self {
            pool: pool.clone(),
            cached_user_loader: CachedUserLoader::new(CachedUserBatcher::new(pool.clone()))
                .with_yield_count(100),
        }
    }
}

impl juniper::Context for Context {}
