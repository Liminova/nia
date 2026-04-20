use gpui::{
    App, AppContext, BorrowAppContext, Context, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, MouseButton, MouseUpEvent, ParentElement, Render, Styled,
    Window, black, div, rgb, white,
};
use nia_ui::components::text_input::TextInput;

use crate::{AppState, NavTarget, NavigateTo};

pub struct LoginScreen {
    pub server_input: Entity<TextInput>,
    pub username_input: Entity<TextInput>,
    pub password_input: Entity<TextInput>,
    pub focus_handle: FocusHandle,
}

impl EventEmitter<NavigateTo> for LoginScreen {}

impl Focusable for LoginScreen {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl LoginScreen {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let server_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Server".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            masked: false,
        });

        let username_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Username".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            masked: false,
        });

        let password_input = cx.new(|cx| TextInput {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Password".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            masked: true,
        });

        cx.new(|cx| Self {
            server_input,
            username_input,
            password_input,
            focus_handle: cx.focus_handle(),
        })
    }

    fn on_submit_click(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let server = self.server_input.read(cx).content.clone().to_string();
        let username = self.username_input.read(cx).content.clone().to_string();
        let password = self.password_input.read(cx).content.clone().to_string();

        cx.update_global(|state: &mut AppState, _| {
            state.base_url = server.clone();
        });

        let client = cx.http_client();

        cx.spawn(async move |this, cx| {
            let login = nia_navidrome::auth::login(client, server, username, password);

            match login.await {
                Ok(creds) => {
                    cx.update_global::<AppState, _>(|state, _| {
                        creds
                            .save(&whoami::username().unwrap_or_else(|_| "nia".to_string()))
                            .ok();
                        state.credentials = Some(creds);
                    })
                    .ok();

                    this.update(cx, |_, cx| {
                        cx.emit(NavigateTo(NavTarget::Main));
                    })
                    .ok()
                }
                Err(e) => {
                    eprintln!("login failed: {e}");
                    Some(())
                }
            }
        })
        .detach();
    }
}

impl Render for LoginScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0xaaaaaa))
            .track_focus(&self.focus_handle(cx))
            .flex()
            .flex_col()
            .size_full()
            .child(self.server_input.clone())
            .child(self.username_input.clone())
            .child(self.password_input.clone())
            .child(
                div()
                    .bg(white())
                    .border_b_1()
                    .border_color(black())
                    .flex()
                    .child("Login")
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::on_submit_click)),
            )
    }
}
