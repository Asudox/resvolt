//! A cache containing data received from the API.
//!
//! ## Why use cache?
//! Using caching reduces latency to access data and allows you to avoid requests to the API.

use ahash::AHashMap;
use {async_trait::async_trait, tokio::sync::RwLock};

use crate::{
    models::{events::ServerEvent, Channel, Member, MemberID, Server, User, ID},
    Context,
};

/// A cache containing data received from the API.
#[derive(Debug, Default)]
pub struct Cache {
    pub(crate) users: RwLock<AHashMap<ID, User>>,
    pub(crate) channels: RwLock<AHashMap<ID, Channel>>,
    pub(crate) servers: RwLock<AHashMap<ID, Server>>,
    pub(crate) members: RwLock<AHashMap<MemberID, Member>>,
}

impl Cache {
    pub(crate) async fn update(ctx: &Context, event: &ServerEvent) {
        match event {
            ServerEvent::Ready(event) => event.update(ctx).await,
            ServerEvent::Message(event) => event.update(ctx).await,
            ServerEvent::ChannelCreate(event) => event.update(ctx).await,
            ServerEvent::ChannelUpdate(event) => event.update(ctx).await,
            ServerEvent::ChannelDelete(event) => event.update(ctx).await,
            ServerEvent::ChannelGroupJoin(event) => event.update(ctx).await,
            ServerEvent::ChannelGroupLeave(event) => event.update(ctx).await,
            ServerEvent::ServerUpdate(event) => event.update(ctx).await,
            ServerEvent::ServerDelete(event) => event.update(ctx).await,
            ServerEvent::ServerMemberUpdate(event) => event.update(ctx).await,
            ServerEvent::ServerMemberJoin(event) => event.update(ctx).await,
            ServerEvent::ServerMemberLeave(event) => event.update(ctx).await,
            ServerEvent::UserUpdate(event) => event.update(ctx).await,
            _ => (),
        }
    }

    /// Get a user from cache.
    pub async fn user(&self, id: &ID) -> Option<User> {
        self.users.read().await.get(id).cloned()
    }

    /// Get all users in cache.
    pub async fn users(&self) -> AHashMap<ID, User> {
        self.users.read().await.clone()
    }

    /// Filter users in cache.
    pub async fn filter_users(&self, filter: impl Fn(&User) -> bool) -> Vec<User> {
        self.users
            .read()
            .await
            .iter()
            .filter_map(|(_, user)| {
                if filter(user) {
                    return Some(user);
                }

                None
            })
            .cloned()
            .collect()
    }

    /// Get a channel from cache.
    pub async fn channel(&self, id: &ID) -> Option<Channel> {
        self.channels.read().await.get(id).cloned()
    }

    /// Get all channels in cache.
    pub async fn channels(&self) -> AHashMap<ID, Channel> {
        self.channels.read().await.clone()
    }

    /// Filter channels in cache.
    pub async fn filter_channels(&self, filter: impl Fn(&Channel) -> bool) -> Vec<Channel> {
        self.channels
            .read()
            .await
            .iter()
            .filter_map(|(_, channel)| {
                if filter(channel) {
                    return Some(channel);
                }

                None
            })
            .cloned()
            .collect()
    }

    /// Get a server from cache.
    pub async fn server(&self, id: &ID) -> Option<Server> {
        self.servers.read().await.get(id).cloned()
    }

    /// Get all servers in cache.
    pub async fn servers(&self) -> AHashMap<ID, Server> {
        self.servers.read().await.clone()
    }

    /// Filter servers in cache.
    pub async fn filter_servers(&self, filter: impl Fn(&Server) -> bool) -> Vec<Server> {
        self.servers
            .read()
            .await
            .iter()
            .filter_map(|(_, server)| {
                if filter(server) {
                    return Some(server);
                }

                None
            })
            .cloned()
            .collect()
    }

    /// Returns the number of servers in cache.
    pub async fn servers_count(&self) -> usize {
        self.servers.read().await.len()
    }

    /// Get a member from cache.
    pub async fn member(&self, id: &MemberID) -> Option<Member> {
        self.members.read().await.get(id).cloned()
    }

    /// Get all members in cache.
    pub async fn members(&self) -> AHashMap<MemberID, Member> {
        self.members.read().await.clone()
    }

    /// Filter members in cache.
    pub async fn filter_members(&self, filter: impl Fn(&Member) -> bool) -> Vec<Member> {
        self.members
            .read()
            .await
            .iter()
            .filter_map(|(_, member)| {
                if filter(member) {
                    return Some(member);
                }

                None
            })
            .cloned()
            .collect()
    }
}

#[async_trait]
pub(crate) trait UpdateCache {
    async fn update(&self, ctx: &Context);
}
