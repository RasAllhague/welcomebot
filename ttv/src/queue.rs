use serde::{Deserialize, Serialize};
use twitch_api::{
    eventsub::{
        channel::{
            ChannelBanV1Payload, ChannelChatClearV1Payload, ChannelChatMessageDeleteV1Payload,
            ChannelUnbanV1Payload, ChannelWarningSendV1Payload,
        },
        stream::{StreamOfflineV1Payload, StreamOnlineV1Payload},
    },
    types::Timestamp,
};

/// Represents events that the Twitch bot can handle.
///
/// This enum defines various types of events that the bot can process, such as
/// stream status changes, chat moderation actions, and warnings.
///
/// Each variant contains the payload associated with the event and a timestamp
/// indicating when the event occurred.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BotEvent {
    /// Event triggered when a stream goes online.
    ///
    /// # Fields
    /// - `StreamOnlineV1Payload`: The payload containing details about the stream.
    /// - `Timestamp`: The timestamp when the event occurred.
    StreamOnline(StreamOnlineV1Payload, Timestamp),

    /// Event triggered when a stream goes offline.
    ///
    /// # Fields
    /// - `StreamOfflineV1Payload`: The payload containing details about the stream.
    /// - `Timestamp`: The timestamp when the event occurred.
    StreamOffline(StreamOfflineV1Payload, Timestamp),

    /// Event triggered when a user is banned from a channel.
    ///
    /// # Fields
    /// - `ChannelBanV1Payload`: The payload containing details about the ban.
    /// - `Timestamp`: The timestamp when the event occurred.
    Ban(ChannelBanV1Payload, Timestamp),

    /// Event triggered when a user is unbanned from a channel.
    ///
    /// # Fields
    /// - `ChannelUnbanV1Payload`: The payload containing details about the unban.
    /// - `Timestamp`: The timestamp when the event occurred.
    Unban(ChannelUnbanV1Payload, Timestamp),

    /// Event triggered when a specific chat message is deleted.
    ///
    /// # Fields
    /// - `ChannelChatMessageDeleteV1Payload`: The payload containing details about the deleted message.
    /// - `Timestamp`: The timestamp when the event occurred.
    MessageDelete(ChannelChatMessageDeleteV1Payload, Timestamp),

    /// Event triggered when the chat is cleared.
    ///
    /// # Fields
    /// - `ChannelChatClearV1Payload`: The payload containing details about the chat clear action.
    /// - `Timestamp`: The timestamp when the event occurred.
    ChatClear(ChannelChatClearV1Payload, Timestamp),

    /// Event triggered when a warning is sent in the chat.
    ///
    /// # Fields
    /// - `ChannelWarningSendV1Payload`: The payload containing details about the warning.
    /// - `Timestamp`: The timestamp when the event occurred.
    Warning(ChannelWarningSendV1Payload, Timestamp),
}
