use async_trait::async_trait;

use crate::Context;
use anyhow::Error;

/// Trait for handling [`anyhow::Error`]] errors
#[async_trait]
pub trait RevoltErrorHandler: Send + Sync + 'static {
    /// Handles the [`anyhow::Error`]
    async fn handle(&self, ctx: &Context, error: Error);
}
