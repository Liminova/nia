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
    pub fn save(&self, user: &str) -> anyhow::Result<()> {
        let creds = serde_json::to_string(&self)?;

        let entry = keyring::Entry::new("nia", user)?;
        entry.set_password(&creds)?;

        Ok(())
    }

    pub fn load(user: &str) -> anyhow::Result<Self> {
        let entry = keyring::Entry::new("nia", user)?;
        let creds = entry.get_password()?;
        let creds: NavidromeCredentials = serde_json::from_str(&creds)?;

        Ok(creds)
    }

    pub fn logout(user: &str) -> anyhow::Result<()> {
        let entry = keyring::Entry::new("nia", user)?;
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
