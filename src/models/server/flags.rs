use bitflags::bitflags;
use serde::Deserialize;

bitflags! {
    /// Server flags.
    #[derive(Deserialize, Default, Debug, Clone, PartialEq)]
    #[serde(transparent)]
    pub struct ServerFlags: u8 {
        /// Official Revolt server.
        const OFFICIAL_REVOLT_SERVER = 1;
        /// Verified community server.
        const VERIFIED_COMMUNITY_SERVER = 2;
    }
}
