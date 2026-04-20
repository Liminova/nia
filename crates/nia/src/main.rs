use std::sync::Arc;

use gpui::prelude::*;
use gpui::{
    Application, Entity, EventEmitter, FocusHandle, Focusable, Global, KeyBinding, MouseButton,
    MouseUpEvent, Subscription, Window, WindowOptions, black, div, rgb, white,
};
use gpui_tokio::Tokio;
use nia_navidrome::auth::NavidromeCredentials;
use nia_ui::components::text_input::{
    Backspace, Cut, Delete, End, Home, Left, Paste, Quit, Right, SelectAll, SelectLeft,
    SelectRight, ShowCharacterPalette, TextInput,
};
use reqwest_client::ReqwestClient;

struct AppState {
    base_url: String,
    credentials: Option<NavidromeCredentials>,
}

impl Global for AppState {}

#[derive(Clone)]
enum Screen {
    Login(Entity<LoginScreen>),
    Main,
}

#[derive(Clone)]
struct NavigateTo(Screen);

impl EventEmitter<NavigateTo> for RootView {}

struct RootView {
    screen: Screen,
    _subscriptions: Vec<Subscription>,
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match self.screen {
            Screen::Login(ref login) => login.clone().into_any_element(),
            Screen::Main => div().size_full().bg(black()).into_any_element(),
        }
    }
}

struct LoginScreen {
    server_input: Entity<TextInput>,
    username_input: Entity<TextInput>,
    password_input: Entity<TextInput>,
    focus_handle: FocusHandle,
}

impl EventEmitter<NavigateTo> for LoginScreen {}

impl Focusable for LoginScreen {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl LoginScreen {
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
                        state.credentials = Some(creds);
                    })
                    .ok();

                    this.update(cx, |_, cx| {
                        cx.emit(NavigateTo(Screen::Main));
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

fn main() {
    Application::new().run(|cx| {
        gpui_tokio::init(cx);
        let http = {
            let _guard = Tokio::handle(cx).enter();

            ReqwestClient::new()
        };
        cx.set_http_client(Arc::new(http));

        let state = AppState {
            base_url: String::new(),
            credentials: None,
        };

        cx.set_global::<AppState>(state);

        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
        ]);

        let window = cx
            .open_window(
                WindowOptions {
                    ..Default::default()
                },
                |_, cx| {
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

                    let login_screen = cx.new(|cx| LoginScreen {
                        server_input,
                        username_input,
                        password_input,
                        focus_handle: cx.focus_handle(),
                    });

                    cx.new(|cx| {
                        let mut subscriptions = vec![];
                        subscriptions.push(cx.subscribe(
                            &login_screen,
                            |root: &mut RootView, _emitter, event, cx| {
                                root.screen = event.0.clone();
                                cx.notify();
                            },
                        ));

                        RootView {
                            screen: Screen::Login(login_screen.clone()),
                            _subscriptions: subscriptions,
                        }
                    })
                },
            )
            .unwrap();

        cx.on_keyboard_layout_change({
            move |cx| {
                window.update(cx, |_, _, cx| cx.notify()).ok();
            }
        })
        .detach();

        window
            .update(cx, |view, window, cx| {
                if let Screen::Login(login) = &view.screen {
                    window.focus(&login.read(cx).server_input.focus_handle(cx));
                }
                cx.activate(true);
            })
            .unwrap();
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    });
}
