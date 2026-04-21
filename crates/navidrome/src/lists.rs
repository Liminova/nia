use std::sync::Arc;

use futures::AsyncReadExt;
use gpui_http_client::{AsyncBody, HttpClient};

use crate::auth::NavidromeCredentials;
use crate::models::{AlbumListResponse, NowPlayingResponse, SubsonicResponse};

pub async fn get_album_list(
    client: Arc<dyn HttpClient>,
    server: String,
    credentials: NavidromeCredentials,
    list_type: String,
) -> anyhow::Result<SubsonicResponse<AlbumListResponse>> {
    let url = format!("{}/rest/getAlbumList", &server);
    let params = [
        ("u", credentials.username.clone()),
        ("t", credentials.token.clone()),
        ("s", credentials.salt.clone()),
        ("v", String::from("2026.4")),
        ("c", String::from("nia")),
        ("f", String::from("json")),
        ("type", list_type),
    ];

    let url = reqwest::Url::parse_with_params(&url, params).unwrap();
    let mut resp = client.get(url.as_ref(), AsyncBody::empty(), true).await?;

    let mut body = vec![];
    resp.body_mut().read_to_end(&mut body).await?;
    let data: SubsonicResponse<AlbumListResponse> = serde_json::from_slice(&body)?;

    Ok(data)
}

pub async fn get_now_playing(
    client: Arc<dyn HttpClient>,
    server: String,
    credentials: NavidromeCredentials,
) -> anyhow::Result<SubsonicResponse<NowPlayingResponse>> {
    let url = format!("{}/rest/getNowPlaying", &server);
    let params = [
        ("u", credentials.username.clone()),
        ("t", credentials.token.clone()),
        ("s", credentials.salt.clone()),
        ("v", String::from("2026.4")),
        ("c", String::from("nia")),
        ("f", String::from("json")),
    ];

    let url = reqwest::Url::parse_with_params(&url, params).unwrap();
    let mut resp = client.get(url.as_ref(), AsyncBody::empty(), true).await?;

    let mut body = vec![];
    resp.body_mut().read_to_end(&mut body).await?;
    let data: SubsonicResponse<NowPlayingResponse> = serde_json::from_slice(&body)?;

    Ok(data)
}
