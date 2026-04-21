use serde::{Deserialize, Serialize};

pub mod agent;
pub mod album;
pub mod artist;
pub mod auth;
pub mod now_playing;
pub mod song;

pub use agent::*;
pub use album::*;
pub use artist::*;
pub use auth::*;
pub use now_playing::*;
pub use song::*;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordLabel {
    pub name: String,
}

pub type ItemGenre = RecordLabel;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDate {
    pub year: Option<u64>,
    pub month: Option<u64>,
    pub day: Option<u64>,
}
