use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub team_id: String,
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

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Channel {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Message {
    bot_id: String,
    r#type: String,
    text: String,
    user: String,
    ts: String,
    app_id: String,
    blocks: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct State {
    values: serde_json::Value,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Action {
    r#type: String,
    block_id: String,
    action_id: String,
    value: String,
    action_ts: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Enterprise {
    id: String,
    name: Option<String>,
}
