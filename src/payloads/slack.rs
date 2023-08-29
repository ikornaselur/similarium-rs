use crate::payloads::common::{Action, Channel, Message, User};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CommandPayload {
    pub team_id: String,
    pub channel_id: String,
    pub user_id: String,
    pub text: String,
    pub api_app_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventPayload {
    // r#type: String,
    pub user: User,
    pub api_app_id: String,
    pub token: String,
    // container: Container,
    // trigger_id: String,
    // team: Team,
    // enterprise: Option<Enterprise>,
    // is_enterprise_install: bool,
    pub channel: Channel,
    pub message: Message,
    // state: State,
    // response_url: String,
    pub actions: Vec<Action>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub payload: String,
}
