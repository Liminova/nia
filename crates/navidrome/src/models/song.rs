use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SongEntry {
    pub id: String,
    pub parent: Option<String>,
    pub album: Option<String>,
    pub title: String,
    pub is_dir: bool,
    pub is_video: Option<bool>,
    pub cover_art: String,
    pub created: String,
    pub duration: u64,
    pub artist_id: String,
    pub artist: String,
    pub year: u64,
    pub genre: Option<String>,
    pub bit_rate: u64,
    pub bit_depth: u64,
    pub sampling_rate: u64,
    pub channel_count: u64,
    pub track: Option<u64>,
    pub size: u64,
    pub disc_number: Option<u64>,
    pub suffix: String,
    pub content_type: String,
    pub path: String,
}
