use crate::auth::NavidromeCredentials;

pub fn get_stream_url(
    server: String,
    credentials: NavidromeCredentials,
    song_id: String,
) -> String {
    let url = format!("{}/rest/stream", &server);
    let params = [
        ("u", credentials.username.clone()),
        ("t", credentials.token.clone()),
        ("s", credentials.salt.clone()),
        ("v", String::from("2026.4")),
        ("c", String::from("nia")),
        ("f", String::from("json")),
        ("id", song_id.clone()),
    ];

    let url = reqwest::Url::parse_with_params(&url, params).unwrap();

    url.to_string()
}
