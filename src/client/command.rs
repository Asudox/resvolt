use anyhow::Result;
use async_trait::async_trait;

use crate::Context;

/// A trait for commands
#[async_trait]
pub trait RevoltCommand: Send + Sync + 'static {
    /// The command code
    async fn execute(&self, ctx: &Context) -> Result<()>;
}
