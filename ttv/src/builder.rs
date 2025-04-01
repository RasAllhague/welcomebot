use crate::{bot::TtvBot, error::Error};


pub struct TtvBotBuilder {
    client_id: twitch_oauth2::ClientId,
    broadcaster_logins: Vec<twitch_api::types::UserName>,
}

impl TtvBotBuilder {
    pub fn new(client_id: twitch_oauth2::ClientId) -> Self {
        Self {
            client_id,
            broadcaster_logins: Vec::new(),
        }
    }

    pub fn add_broadcaster_login(mut self, login: twitch_api::types::UserName) -> Self {
        self.broadcaster_logins.push(login);
        self
    }

    pub async fn build(mut self) -> Result<TtvBot, Error> {
        todo!()
    } 
}