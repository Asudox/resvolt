use serde::Deserialize;

use crate::models::{Channel, Member, Server, User};

#[cfg(feature = "cache")]
use crate::{cache::UpdateCache, Context};

/// Bot is ready.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ReadyEvent {
    /// Users.
    pub users: Vec<User>,
    /// Servers.
    pub servers: Vec<Server>,
    /// Channels.
    pub channels: Vec<Channel>,
    /// Members.
    pub members: Vec<Member>,
    // Add Optional emojis field
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ReadyEvent {
    async fn update(&self, ctx: &Context) {
        let mut users = ctx.cache.users.write().await;

        for user in &self.users {
            users.insert(user.id.clone(), user.clone());
        }

        let mut channels = ctx.cache.channels.write().await;

        for channel in &self.channels {
            channels.insert(channel.id().clone(), channel.clone());
        }

        let mut servers = ctx.cache.servers.write().await;

        for server in &self.servers {
            servers.insert(server.id.clone(), server.clone());
        }

        let mut members = ctx.cache.members.write().await;

        for member in &self.members {
            members.insert(member.id.clone(), member.clone());
        }
    }
}
