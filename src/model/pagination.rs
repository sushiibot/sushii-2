use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

use crate::error::Result;

pub struct Paginator<T, O: Clone> {
    page_data: Vec<T>,
    page_size: usize,
    page_count: usize,
    item_count: usize,
    current_page: usize,
    offsets: Vec<Option<O>>,
}

impl<T, O: Clone> Paginator<T, O> {
    pub fn new(page_data: Vec<T>, item_count: usize) -> Self {
        let page_size = 10;
        let page_count = (item_count + page_size - 1) / page_size;

        Self {
            page_data,
            page_size,
            page_count,
            item_count,
            current_page: 1,
            offsets: vec![None; page_count],
        }
    }

    pub fn page_size(mut self, page_size: usize) -> Self {
        self.page_size = page_size;
        self
    }
}

#[async_trait]
pub trait PaginateQuery {
    type Output;

    async fn get_page(
        ctx: &Context,
        guild_id: GuildId,
        count: i64,
        offset: Option<&str>,
    ) -> Result<Vec<Self::Output>>;
}

// pub type FutReturn<T> = Pin<Box<dyn Future<Output = Result<Vec<T>>> + Send>> + Send + Sync;

#[async_trait]
pub trait Paginate<T, O> {
    async fn next<F1, F2, Fut>(&mut self, get_offset: F1, get_next_page: F2) -> Result<bool>
    where
        F1: FnOnce(&T) -> O + Send + Sync,
        F2: Fn(Option<O>) -> Pin<Box<dyn Future<Output = Result<Vec<T>>> + Send>> + Send + Sync;

    async fn prev<F1>(&mut self, get_prev_page: F1) -> Result<bool>
    where
        F1: Fn(Option<O>) -> Pin<Box<dyn Future<Output = Result<Vec<T>>> + Send>> + Send + Sync;
}

#[async_trait]
impl<T: Send, O: Clone + Send> Paginate<T, O> for Paginator<T, O> {
    async fn next<F1, F2, Fut>(&mut self, get_offset: F1, get_next_page: F2) -> Result<bool>
    where
        F1: FnOnce(&T) -> O + Send + Sync,
        F2: Fn(Option<O>) -> Pin<Box<dyn Future<Output = Result<Vec<T>>> + Send>> + Send + Sync
    {
        if self.page_count == self.current_page {
            return Ok(false);
        }

        // Use the last item in current page as offset
        let offset = self.page_data.last().map(get_offset);
        self.offsets[self.current_page] = offset.clone();

        // Get next page
        self.page_data = get_next_page(offset).await?;
        self.current_page += 1;

        Ok(true)
    }

    async fn prev<F1>(&mut self, get_prev_page: F1) -> Result<bool>
    where
        F1: Fn(Option<O>) -> Pin<Box<dyn Future<Output = Result<Vec<T>>> + Send>> + Send + Sync
    {
        // Ignore on first page
        if self.current_page == 1 {
            return Ok(false);
        }

        // Use previous page's last tag as offset
        let offset = self.offsets[self.current_page - 2].clone();
        self.page_data = get_prev_page(offset).await?;
        self.current_page -= 1;

        Ok(true)
    }
}
