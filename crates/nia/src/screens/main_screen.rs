use gpui::{
    App, AppContext, Context, Entity, EventEmitter, IntoElement, ParentElement, Render, Styled,
    Window, div, rgb,
};
use nia_navidrome::lists::get_album_list;
use nia_navidrome::models::AlbumEntry;

use crate::{AppState, NavigateTo};

pub struct MainScreen {
    albums: Option<Vec<AlbumEntry>>,
}

impl MainScreen {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let state = cx.global::<AppState>();
            let client = cx.http_client();
            let server = state.base_url.clone();
            let credentials = state.credentials.clone().unwrap();

            let entity = Self { albums: None };

            cx.spawn(async move |this, cx| {
                // TODO: hardcoding this to "recent" for testing, will impl the rest later
                match get_album_list(client, server, credentials, String::from("recent")).await {
                    Ok(resp) => this
                        .update(cx, |screen: &mut Self, cx| {
                            screen.albums =
                                Some(resp.inner_subsonic_response.body.album_list.album);
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

impl EventEmitter<NavigateTo> for MainScreen {}

impl Render for MainScreen {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match &self.albums {
            None => div().bg(rgb(0xaaaaaa)).size_full(),
            Some(albums) => div()
                .bg(rgb(0xaaaaaa))
                .size_full()
                .flex()
                .flex_col()
                .children(albums.iter().map(|a| div().child(a.title.clone()))),
        }
    }
}
