use serde::Deserialize;

use crate::{
    models::{Channel, User, ID},
    Context,
};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// A user has joined the group.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelGroupJoinEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: ID,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: ID,
}

impl ChannelGroupJoinEvent {
    /// Fetch the channel.
    pub async fn channel(&self, ctx: &Context) -> Result<Channel> {
        Channel::fetch(ctx, &self.channel_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, ctx: &Context) -> Result<User> {
        User::fetch(ctx, &self.user_id).await
    }
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ChannelGroupJoinEvent {
    async fn update(&self, ctx: &Context) {
        if let Some(Channel::Group(ref mut group)) =
            ctx.cache.channels.write().await.get_mut(&self.channel_id)
        {
            group.recipients.push(self.user_id.clone());
        }
    }
}
