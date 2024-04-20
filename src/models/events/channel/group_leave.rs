use serde::Deserialize;

use crate::{
    models::{Channel, User, ID},
    Context,
};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// A user has left the group.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ChannelGroupLeaveEvent {
    /// Channel id.
    #[serde(rename = "id")]
    pub channel_id: ID,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: ID,
}

impl ChannelGroupLeaveEvent {
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
impl UpdateCache for ChannelGroupLeaveEvent {
    async fn update(&self, ctx: &Context) {
        if let Some(Channel::Group(ref mut channel)) =
            ctx.cache.channels.write().await.get_mut(&self.channel_id)
        {
            if let Some(index) = channel
                .recipients
                .iter()
                .position(|user_id| *user_id == self.user_id)
            {
                channel.recipients.remove(index);
            }
        }
    }
}
