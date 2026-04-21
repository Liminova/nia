use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistID3 {
    pub id: String,
    pub name: String,
    pub cover_art: Option<String>,
    pub artist_image_url: Option<String>,
    pub album_count: Option<u64>,
    pub starred: Option<String>,
    pub music_brainz_id: Option<String>,
    pub sort_name: Option<String>,
    pub roles: Option<Vec<String>>,
}
