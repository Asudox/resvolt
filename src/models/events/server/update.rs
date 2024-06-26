use serde::{Deserialize, Serialize};

use crate::{
    models::{Attachment, Category, Server, SystemMessageChannels, ID},
    Context,
};

use anyhow::Result;

#[cfg(feature = "cache")]
use crate::cache::UpdateCache;

/// Specifies a field to remove on server update.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum ServerField {
    /// Server icon.
    Icon,
    /// Server banner.
    Banner,
    /// Server description.
    Description,
}

/// A server details were updated.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ServerUpdateEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: ID,
    /// A partial server object.
    pub data: PartialServer,
    /// A specified field to remove on server update.
    pub clear: Option<ServerField>,
}

impl ServerUpdateEvent {
    /// Fetch the server.
    pub async fn server(&self, ctx: &Context) -> Result<Server> {
        Server::fetch(ctx, &self.server_id).await
    }
}

/// A partial server.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PartialServer {
    /// Server name.
    pub name: Option<String>,
    /// Server description.
    pub description: Option<String>,
    /// Server icon.
    pub icon: Option<Attachment>,
    /// Server banner.
    pub banner: Option<Attachment>,
    /// Server categories.
    #[serde(default)]
    pub categories: Vec<Category>,
    /// Server system message channels.
    pub system_messages: Option<SystemMessageChannels>,
    /// Whether server is not safe for work.
    pub nsfw: Option<bool>,
}

#[cfg(feature = "cache")]
#[async_trait::async_trait]
impl UpdateCache for ServerUpdateEvent {
    async fn update(&self, ctx: &Context) {
        if let Some(server) = ctx.cache.servers.write().await.get_mut(&self.server_id) {
            if let Some(field) = self.clear {
                match field {
                    ServerField::Icon => server.icon = None,
                    ServerField::Banner => server.banner = None,
                    ServerField::Description => server.description = None,
                }
            }

            if let Some(ref name) = self.data.name {
                server.name = name.clone();
            }

            if let Some(ref description) = self.data.description {
                server.description = Some(description.clone());
            }

            if let Some(ref icon) = self.data.icon {
                server.icon = Some(icon.clone());
            }

            if let Some(ref banner) = self.data.banner {
                server.banner = Some(banner.clone());
            }

            if !self.data.categories.is_empty() {
                server.categories = self.data.categories.clone();
            }

            if let Some(ref system_messages) = self.data.system_messages {
                server.system_messages = system_messages.clone();
            }

            if let Some(nsfw) = self.data.nsfw {
                server.nsfw = nsfw;
            }
        }
    }
}
