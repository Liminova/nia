use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div, rgb,
};
use nia_navidrome::browsing::get_album_info;
use nia_navidrome::models::AlbumID3;

use crate::AppState;

pub struct AlbumScreen {
    album: Option<AlbumID3>,
}

impl AlbumScreen {
    pub fn new(cx: &mut App, album_id: String) -> Entity<Self> {
        cx.new(|cx| {
            let state = cx.global::<AppState>();
            let client = cx.http_client();
            let server = state.base_url.clone();
            let credentials = state.credentials.clone().unwrap();

            let entity = Self { album: None };

            cx.spawn(async move |this, cx| {
                // TODO: hardcoding this to "recent" for testing, will impl the rest later
                match get_album_info(client, server, credentials, album_id).await {
                    Ok(resp) => this
                        .update(cx, |screen: &mut Self, cx| {
                            screen.album = Some(resp.inner_subsonic_response.body.album);
                            cx.notify();
                        })
                        .ok(),
                    Err(e) => {
                        tracing::error!("failed to fetch albums: {e}");
                        Some(())
                    }
                }
            })
            .detach();

            entity
        })
    }
}

impl Render for AlbumScreen {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match &self.album {
            Some(album) => {
                let songs = album.song.as_ref().map(|songs| {
                    songs
                        .iter()
                        .map(|song| div().child(song.title.clone()))
                        .collect::<Vec<_>>()
                });

                div()
                    .bg(rgb(0xaaaaaa))
                    .size_full()
                    .flex()
                    .flex_col()
                    .child(album.name.clone())
                    .child(album.artist.clone().unwrap_or_default())
                    .child(div().flex_col().children(songs.unwrap_or_default()))
            }
            None => div().bg(rgb(0xaaaaaa)).size_full(),
        }
    }
}
