use std::sync::Arc;

use futures::AsyncReadExt;
use gpui_http_client::{AsyncBody, HttpClient};

use crate::auth::NavidromeCredentials;
use crate::models::{AlbumResponse, SubsonicResponse};

pub async fn get_album_info(
    client: Arc<dyn HttpClient>,
    server: String,
    credentials: NavidromeCredentials,
    id: String,
) -> anyhow::Result<SubsonicResponse<AlbumResponse>> {
    let url = format!("{}/rest/getAlbum", &server);
    let params = [
        ("u", credentials.username.clone()),
        ("t", credentials.token.clone()),
        ("s", credentials.salt.clone()),
        ("v", String::from("2026.4")),
        ("c", String::from("nia")),
        ("f", String::from("json")),
        ("id", id.clone()),
    ];

    let url = reqwest::Url::parse_with_params(&url, params).unwrap();
    let mut resp = client.get(url.as_ref(), AsyncBody::empty(), true).await?;

    let mut body = vec![];
    resp.body_mut().read_to_end(&mut body).await?;
    let data: SubsonicResponse<AlbumResponse> = serde_json::from_slice(&body)?;

    Ok(data)
}
