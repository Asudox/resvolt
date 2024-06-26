use {serde::Deserialize, serde_json::Value as Json};

use crate::{
    models::{Server, ID},
    Context,
};

use anyhow::Result;

/// Specifies a field to remove on server role update.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum RoleField {
    /// Role color.
    #[serde(rename = "Colour")]
    Color,
}

/// A server role details were updated.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerRoleUpdateEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: ID,
    /// Server role id.
    pub role_id: ID,
    /// A partial server role object.
    pub data: Json,
    /// A specified field to remove on server role update.
    pub clear: Option<RoleField>,
}

impl ServerRoleUpdateEvent {
    /// Fetch the server.
    pub async fn server(&self, ctx: &Context) -> Result<Server> {
        Server::fetch(ctx, &self.server_id).await
    }
}
