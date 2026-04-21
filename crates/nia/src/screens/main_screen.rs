use gpui::{
    App, AppContext, Context, Entity, EventEmitter, InteractiveElement, IntoElement, ParentElement,
    Render, Styled, Window, div, rgb,
};
use nia_navidrome::lists::get_album_list;
use nia_navidrome::models::AlbumID3;

use crate::screens::AlbumScreen;
use crate::{AppState, NavigateTo};

pub enum MainRoute {
    AlbumList,
    Album(Entity<AlbumScreen>),
}

pub struct MainScreen {
    albums: Option<Vec<AlbumID3>>,
    route: MainRoute,
}

impl MainScreen {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let state = cx.global::<AppState>();
            let client = cx.http_client();
            let server = state.base_url.clone();
            let credentials = state.credentials.clone().unwrap();

            let entity = Self {
                albums: None,
                route: MainRoute::AlbumList,
            };

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

    fn on_album_click(&mut self, album_id: String, cx: &mut Context<Self>) {
        self.route = MainRoute::Album(AlbumScreen::new(cx, album_id.clone()));
        cx.notify();
    }
}

impl EventEmitter<NavigateTo> for MainScreen {}

impl Render for MainScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match &self.route {
            MainRoute::AlbumList => match &self.albums {
                None => div().bg(rgb(0xaaaaaa)).size_full().into_any_element(),
                Some(albums) => div()
                    .bg(rgb(0xaaaaaa))
                    .size_full()
                    .flex()
                    .flex_col()
                    .children(albums.iter().map(|a| {
                        let id = a.id.clone();
                        div().child(a.name.clone()).on_mouse_up(
                            gpui::MouseButton::Left,
                            cx.listener(move |this, _, _window, cx| {
                                this.on_album_click(id.clone(), cx);
                            }),
                        )
                    }))
                    .into_any_element(),
            },
            MainRoute::Album(album) => album.clone().into_any_element(),
        }
    }
}
