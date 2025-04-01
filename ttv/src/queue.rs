use serde::{Deserialize, Serialize};
use twitch_api::{eventsub::{
    channel::{
        ChannelBanV1Payload, ChannelChatClearV1Payload, ChannelChatMessageDeleteV1Payload,
        ChannelUnbanV1Payload, ChannelWarningSendV1Payload,
    },
    stream::{StreamOfflineV1Payload, StreamOnlineV1Payload},
}, types::Timestamp};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BotEvent {
    StreamOnline(StreamOnlineV1Payload, Timestamp),
    StreamOffline(StreamOfflineV1Payload, Timestamp),
    Ban(ChannelBanV1Payload, Timestamp),
    Unban(ChannelUnbanV1Payload, Timestamp),
    MessageDelete(ChannelChatMessageDeleteV1Payload, Timestamp),
    ChatClear(ChannelChatClearV1Payload, Timestamp),
    Warning(ChannelWarningSendV1Payload, Timestamp),
}
