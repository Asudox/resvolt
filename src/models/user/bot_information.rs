use serde::Deserialize;

use crate::models::ID;

/// A bot information.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct BotInformation {
    /// Bot owner id.
    #[serde(rename = "owner")]
    pub owner_id: ID,
}
