use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlayingEntry {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_id: String,
    pub cover_art: String,
    pub duration: u64,
    pub play_count: u64,
    pub created: String,
    pub starred: Option<String>,
    pub year: Option<u64>,
    pub genre: Option<String>,
    pub size: u64,
    pub content_type: String,
    pub suffix: String,
    pub path: String,
    pub played: String,
    pub username: String,
    pub minutes_ago: u64,
    pub player_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerNowPlayingResponse {
    pub entry: Vec<NowPlayingEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlayingResponse {
    pub now_playing: InnerNowPlayingResponse,
}
