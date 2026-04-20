use serde::{Deserialize, Serialize};

pub mod album;
pub mod auth;
pub mod now_playing;

pub use album::*;
pub use auth::*;
pub use now_playing::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct SubsonicResponse<T> {
    #[serde(rename = "subsonic-response")]
    pub inner_subsonic_response: InnerSubsonicResponse<T>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerSubsonicResponse<T> {
    pub status: SubsonicResponseStatus,
    pub version: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub server_version: String,
    pub open_subsonic: bool,
    #[serde(flatten)]
    pub body: T,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubsonicResponseStatus {
    OK,
    FAILED,
}
