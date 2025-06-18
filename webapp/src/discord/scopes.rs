use serde::{Deserialize, Serialize};

/// These are a list of all the OAuth2 scopes that Discord supports. 
/// Some scopes require approval from Discord to use. 
/// Requesting them from a user without approval from Discord may cause errors or undocumented behavior in the OAuth2 flow.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum OAuth2Scope {
    /// Allows your app to fetch data from a user's "Now Playing/Recently Played" list — not currently available for apps
    #[serde(rename = "activities.read")]
    ActivitiesRead,
    /// Allows your app to update a user's activity - not currently available for apps (NOT REQUIRED FOR GAMESDK ACTIVITY MANAGER)
    #[serde(rename = "activities.write")]
    ActivitiesWrite,
    /// Allows your app to read build data for a user's applications
    #[serde(rename = "applications.builds.read")]
    ApplicationsBuildsRead,
    /// Allows your app to upload/update builds for a user's applications - requires Discord approval
    #[serde(rename = "applications.builds.upload")]
    ApplicationsBuildsUpload,
    /// Allows your app to add commands to a guild - included by default with the bot scope
    #[serde(rename = "applications.commands")]
    ApplicationsCommands,
    /// Allows your app to update its commands using a Bearer token - client credentials grant only
    #[serde(rename = "applications.commands.update")]
    ApplicationsCommandsUpdate,
    /// Allows your app to update permissions for its commands in a guild a user has permissions to
    #[serde(rename = "applications.commands.permissions.update")]
    ApplicationsCommandsPermissionsUpdate,
    /// Allows your app to read entitlements for a user's applications
    #[serde(rename = "applications.entitlements")]
    ApplicationsEntitlements,
    /// Allows your app to read and update store data (SKUs, store listings, achievements, etc.) for a user's applications
    #[serde(rename = "applications.store.update")]
    ApplicationsStoreUpdate,
    /// For oauth2 bots, this puts the bot in the user's selected guild by default
    #[serde(rename = "bot")]
    Bot,
    /// Allows /users/@me/connections to return linked third-party accounts
    #[serde(rename = "connections")]
    Connections,
    /// Allows your app to see information about the user's DMs and group DMs - requires Discord approval
    #[serde(rename = "dm_channels.read")]
    DmChannelsRead,
    /// Enables /users/@me to return an email
    #[serde(rename = "email")]
    Email,
    /// Allows your app to join users to a group dm
    #[serde(rename = "gdm.join")]
    GdmJoin,
    /// Allows /users/@me/guilds to return basic information about all of a user's guilds
    #[serde(rename = "guilds")]
    Guilds,
    /// Allows /guilds/{guild.id}/members/{user.id} to be used for joining users to a guild
    #[serde(rename = "guilds.join")]
    GuildsJoin,
    /// Allows /users/@me/guilds/{guild.id}/member to return a user's member information in a guild
    #[serde(rename = "guilds.members.read")]
    GuildsMembersRead,
    /// Allows /users/@me without email
    #[serde(rename = "identify")]
    Identify,
    /// For local rpc server api access, this allows you to read messages from all client channels (otherwise restricted to channels/guilds your app creates)
    #[serde(rename = "messages.read")]
    MessagesRead,
    /// Allows your app to know a user's friends and implicit relationships - requires Discord approval
    #[serde(rename = "relationships.read")]
    RelationshipsRead,
    /// Allows your app to update a user's connection and metadata for the app
    #[serde(rename = "role_connections.write")]
    RoleConnectionsWrite,
    /// For local rpc server access, this allows you to control a user's local Discord client - requires Discord approval
    #[serde(rename = "rpc")]
    Rpc,
    /// For local rpc server access, this allows you to update a user's activity - requires Discord approval
    #[serde(rename = "rpc.activities.write")]
    RpcActivitiesWrite,
    /// For local rpc server access, this allows you to receive notifications pushed out to the user - requires Discord approval
    #[serde(rename = "rpc.notifications.read")]
    RpcNotificationsRead,
    /// For local rpc server access, this allows you to read a user's voice settings and listen for voice events - requires Discord approval
    #[serde(rename = "rpc.voice.read")]
    RpcVoiceRead,
    /// For local rpc server access, this allows you to update a user's voice settings - requires Discord approval
    #[serde(rename = "rpc.voice.write")]
    RpcVoiceWrite,
    /// Allows your app to connect to voice on user's behalf and see all the voice members - requires Discord approval
    #[serde(rename = "voice")]
    Voice,
    /// This generates a webhook that is returned in the oauth token response for authorization code grants
    #[serde(rename = "webhook.incoming")]
    WebhookIncoming,
}

impl ToString for OAuth2Scope {
    fn to_string(&self) -> String {
        match self {
            OAuth2Scope::ActivitiesRead => "activities.read",
            OAuth2Scope::ActivitiesWrite => "activities.write",
            OAuth2Scope::ApplicationsBuildsRead => "applications.builds.read",
            OAuth2Scope::ApplicationsBuildsUpload => "applications.builds.upload",
            OAuth2Scope::ApplicationsCommands => "applications.commands",
            OAuth2Scope::ApplicationsCommandsUpdate => "applications.commands.update",
            OAuth2Scope::ApplicationsCommandsPermissionsUpdate => {
                "applications.commands.permissions.update"
            }
            OAuth2Scope::ApplicationsEntitlements => "applications.entitlements",
            OAuth2Scope::ApplicationsStoreUpdate => "applications.store.update",
            OAuth2Scope::Bot => "bot",
            OAuth2Scope::Connections => "connections",
            OAuth2Scope::DmChannelsRead => "dm_channels.read",
            OAuth2Scope::Email => "email",
            OAuth2Scope::GdmJoin => "gdm.join",
            OAuth2Scope::Guilds => "guilds",
            OAuth2Scope::GuildsJoin => "guilds.join",
            OAuth2Scope::GuildsMembersRead => "guilds.members.read",
            OAuth2Scope::Identify => "identify",
            OAuth2Scope::MessagesRead => "messages.read",
            OAuth2Scope::RelationshipsRead => "relationships.read",
            OAuth2Scope::RoleConnectionsWrite => "role_connections.write",
            OAuth2Scope::Rpc => "rpc",
            OAuth2Scope::RpcActivitiesWrite => "rpc.activities.write",
            OAuth2Scope::RpcNotificationsRead => "rpc.notifications.read",
            OAuth2Scope::RpcVoiceRead => "rpc.voice.read",
            OAuth2Scope::RpcVoiceWrite => "rpc.voice.write",
            OAuth2Scope::Voice => "voice",
            OAuth2Scope::WebhookIncoming => "webhook.incoming",
        }
        .to_string()
    }
}
