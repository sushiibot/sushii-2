use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub pool: Arc<sqlx::PgPool>,
}

impl Context {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }
}

impl juniper::Context for Context {}
