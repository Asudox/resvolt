use serde::Deserialize;

use crate::{
    models::{Server, ID},
    Context,
};

use anyhow::Result;

/// A server role has been deleted.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerRoleDeleteEvent {
    /// Server id.
    #[serde(rename = "id")]
    pub server_id: ID,
    /// Server role id.
    pub role_id: ID,
}

impl ServerRoleDeleteEvent {
    /// Fetch the server.
    pub async fn server(&self, ctx: &Context) -> Result<Server> {
        Server::fetch(ctx, &self.server_id).await
    }
}
