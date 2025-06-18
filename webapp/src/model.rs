use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Store, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32, 
    pub username: String,
    pub avatar: Option<String>,
    pub user_id: i64,
    pub twitch_broadcaster_id: Option<i32>,
}