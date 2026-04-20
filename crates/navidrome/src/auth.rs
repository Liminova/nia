use std::sync::Arc;

use futures::AsyncReadExt;
use gpui_http_client::{AsyncBody, HttpClient};

use crate::models::NavidromeLoginResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NavidromeCredentials {
    pub server: String,
    pub username: String,
    pub token: String,
    pub salt: String,
}

impl NavidromeCredentials {
    pub async fn save(&self, server: &str) -> anyhow::Result<()> {
        let creds = serde_json::to_string(&self)?;

        let entry = keyring::Entry::new("nia", server)?;
        entry.set_password(&creds)?;

        Ok(())
    }

    pub async fn load(server: &str) -> anyhow::Result<Self> {
        let entry = keyring::Entry::new("nia", server)?;
        let creds = entry.get_password()?;
        let creds: NavidromeCredentials = serde_json::from_str(&creds)?;

        Ok(creds)
    }

    pub async fn logout(server: &str) -> anyhow::Result<()> {
        let entry = keyring::Entry::new("nia", server)?;
        entry.delete_credential()?;

        Ok(())
    }
}

pub async fn login(
    client: Arc<dyn HttpClient>,
    server: String,
    username: String,
    password: String,
) -> anyhow::Result<NavidromeCredentials> {
    let mut resp = client
        .post_json(
            &format!("{}/auth/login", server),
            AsyncBody::from(serde_json::to_string(&serde_json::json!({
                "username": username,
                "password": password,
            }))?),
        )
        .await?;

    let mut body = vec![];
    resp.body_mut().read_to_end(&mut body).await?;
    let data: NavidromeLoginResponse = serde_json::from_slice(&body)?;

    let creds = NavidromeCredentials {
        server,
        username,
        token: data.subsonic_token,
        salt: data.subsonic_salt,
    };

    Ok(creds)
}
