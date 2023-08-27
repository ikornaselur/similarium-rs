use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub team_id: String,
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub real_name: String,
    pub profile: Profile,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub avatar_hash: String,
    pub status_text: String,
    pub status_emoji: String,
    pub real_name: String,
    pub display_name: String,
    pub real_name_normalized: String,
    pub display_name_normalized: String,
    pub image_original: String,
    pub image_24: String,
    pub image_32: String,
    pub image_48: String,
    pub image_72: String,
    pub image_192: String,
    pub image_512: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthedUser {
    pub id: String,
    pub scope: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Container {
    pub r#type: String,
    pub message_ts: String,
    pub channel_id: String,
    pub is_ephemeral: bool,
}

#[derive(Deserialize, Debug)]
pub struct Team {
    pub id: String,
    pub domain: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Channel {
    pub id: String,
    //name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    //bot_id: String,
    //r#type: String,
    //text: String,
    //user: String,
    pub ts: String,
    //app_id: String,
    //blocks: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct State {
    values: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Action {
    //r#type: String,
    //block_id: String,
    pub action_id: String,
    pub value: String,
    //action_ts: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Enterprise {
    id: String,
    name: Option<String>,
}
