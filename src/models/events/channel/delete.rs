use serde::Deserialize;

use crate::models::ID;

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// A channel has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelDeleteEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: ID,
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ChannelDeleteEvent {
    async fn update(&self, ctx: &Context) {
        ctx.cache.channels.write().await.remove(&self.channel_id);
    }
}
