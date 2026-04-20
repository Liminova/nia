use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumEntry {
    pub id: String,
    pub parent: String,
    pub album: String,
    pub title: String,
    pub name: String,
    pub is_dir: bool,
    pub cover_art: String,
    pub created: String,
    pub duration: u64,
    pub play_count: u64,
    pub artist_id: String,
    pub artist: String,
    pub year: u64,
    pub genre: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerAlbumResponse {
    pub album: Vec<AlbumEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumResponse {
    pub album_list: InnerAlbumResponse,
}
