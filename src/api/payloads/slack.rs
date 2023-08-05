use crate::api::payloads::common::{
    Action, Channel, Container, Enterprise, Message, State, Team, User,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CommandPayload {
    pub team_id: String,
    pub channel_id: String,
    pub user_id: String,
    pub text: String,
    pub api_app_id: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct EventPayload {
    r#type: String,
    user: User,
    api_app_token: String,
    token: String,
    container: Container,
    trigger_id: String,
    team: Team,
    enterprise: Option<Enterprise>,
    is_enterprise_install: bool,
    channel: Channel,
    message: Message,
    state: State,
    response_url: String,
    actions: Vec<Action>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Event {
    payload: EventPayload,
}
