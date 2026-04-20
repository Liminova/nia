use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NavidromeLoginResponse {
    pub name: String,
    pub username: String,
    pub is_admin: bool,
    pub subsonic_salt: String,
    pub subsonic_token: String,
    pub token: String,
}
