use serde::Deserialize;

use crate::{
    models::{Server, User, ID},
    Context,
};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// A user has left the server.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerMemberLeaveEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: ID,
    /// User id.
    #[serde(rename = "user")]
    pub user_id: ID,
}

impl ServerMemberLeaveEvent {
    /// Fetch the server.
    pub async fn server(&self, ctx: &Context) -> Result<Server> {
        Server::fetch(ctx, &self.server_id).await
    }

    /// Fetch the user.
    pub async fn user(&self, ctx: &Context) -> Result<User> {
        User::fetch(ctx, &self.user_id).await
    }
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ServerMemberLeaveEvent {
    async fn update(&self, ctx: &Context) {
        ctx.cache
            .members
            .write()
            .await
            .remove(&(&self.server_id, &self.user_id).into());
    }
}
