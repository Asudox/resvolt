use bitflags::bitflags;
use serde::Deserialize;

bitflags! {
    /// Enum of revolt permissions
    #[derive(Deserialize, Debug)]
    #[serde(transparent)]
    pub struct RevoltPermission: u64 {
        /// Manage the channel or channels on the server
        const MANAGE_CHANNEL = 1;
        /// Manage the server
        const MANAGE_SERVER = 2;
        /// Manage permissions on servers or channels
        const MANAGE_PERMISSIONS = 4;
        /// Manage roles on server
        const MANAGE_ROLE = 8;
        ///	Manage emoji on servers
        const MANAGE_CUSTOMISATION = 16;
        ///	Kick other members below their ranking
        const KICK_MEMBERS = 64;
        /// Ban other members below their ranking
        const BAN_MEMBERS = 128;
        ///	Timeout other members below their ranking
        const TIMEOUT_MEMBERS = 256;
        ///	Assign roles to members below their ranking
        const ASSIGN_ROLES = 512;
        ///	Change own nickname
        const CHANGE_NICKNAME = 1024;
        /// Change or remove other's nicknames below their ranking
        const MANAGE_NICKNAMES = 2048;
        /// Change own avatar
        const CHANGE_AVATAR = 4096;
        ///	Remove other's avatars below their ranking
        const REMOVE_AVATAR = 8192;
        ///	View a channel
        const VIEW_CHANNEL = 1048576;
        ///	Read a channel's past message history
        const READ_MESSAGE_HISTORY = 2097152;
        /// Send a message in a channel
        const SEND_MESSAGE = 4194304;
        ///	Delete messages in a channel
        const MANAGE_MESSAGES = 8388608;
        ///	Manage webhook entries on a channel
        const MANAGE_WEBHOOKS = 16777216;
        /// Create invites to this channel
        const INVITE_OTHERS = 33554432;
        ///	Send embedded content in this channel
        const SEND_EMBEDS = 67108864;
        ///	Send attachments and media in this channel
        const UPLOAD_FILES = 134217728;
        ///	Masquerade messages using custom nickname and avatar
        const MASQUERADE = 268435456;
        /// React to messages with emojis
        const REACT = 536870912;
        ///	Connect to a voice channel
        const CONNECT = 1073741824;
        ///	Speak in a voice call
        const SPEAK = 2147483648;
        ///	Share video in a voice call
        const VIDEO = 4294967296;
        ///	Mute other members with lower ranking in a voice call
        const MUTE_MEMBERS = 8589934592;
        ///	Deafen other members with lower ranking in a voice call
        const DEAFEN_MEMBERS = 17179869184;
        ///	Move members between voice channels
        const MOVE_MEMBERS = 34359738368;
    }
}
