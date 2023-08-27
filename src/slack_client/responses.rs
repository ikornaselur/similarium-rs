use crate::payloads::{Team, UserInfo};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SlackOAuthResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub access_token: Option<String>,
    pub scope: Option<String>,
    pub bot_user_id: Option<String>,
    pub app_id: String,
    pub team: Team,
    pub is_enterprise_install: bool,
}

#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub user: Option<UserInfo>,
}
