use serde::{Deserialize, Serialize};

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
