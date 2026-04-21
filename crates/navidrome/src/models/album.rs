use serde::{Deserialize, Serialize};

use crate::models::{ArtistID3, ItemDate, ItemGenre, RecordLabel, SongEntry};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumID3 {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub artist: Option<String>,
    pub artist_id: Option<String>,
    pub cover_art: Option<String>,
    pub song_count: u64,
    pub duration: u64,
    pub play_count: Option<u64>,
    pub created: String,
    pub starred: Option<String>,
    pub year: Option<u64>,
    pub genre: Option<Genre>,
    pub played: String,
    pub user_rating: Option<u8>,
    pub record_labels: Option<Vec<RecordLabel>>,
    pub music_brainz_id: String,
    pub genres: Vec<ItemGenre>,
    pub artists: Vec<ArtistID3>,
    pub display_artist: String,
    pub release_types: Option<Vec<String>>,
    pub moods: Vec<String>,
    pub sort_name: String,
    pub original_release_date: Option<ItemDate>,
    pub release_date: Option<ItemDate>,
    pub is_compilation: Option<bool>,
    pub explicit_status: String,
    pub disc_titles: Option<Vec<DiscTitle>>,
    pub song: Option<Vec<SongEntry>>,
}

pub type Genre = String;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscTitle {
    pub disc: u64,
    pub title: String,
    pub cover_art: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerAlbumListResponse {
    pub album: Vec<AlbumID3>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumListResponse {
    pub album_list: InnerAlbumListResponse,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumResponse {
    pub album: AlbumID3,
}
