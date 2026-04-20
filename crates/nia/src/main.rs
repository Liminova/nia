mod screens;

use std::sync::Arc;

use gpui::prelude::*;
use gpui::{
    Application, Entity, EventEmitter, Focusable, Global, KeyBinding, Subscription, Window,
    WindowOptions,
};
use gpui_tokio::Tokio;
use nia_navidrome::auth::{NavidromeCredentials, login};
use nia_ui::components::text_input::{
    Backspace, Cut, Delete, End, Home, Left, Paste, Quit, Right, SelectAll, SelectLeft,
    SelectRight, ShowCharacterPalette,
};
use reqwest_client::ReqwestClient;

use crate::screens::{LoginScreen, MainScreen};

struct AppState {
    base_url: String,
    credentials: Option<NavidromeCredentials>,
}

impl Global for AppState {}

#[derive(Clone)]
enum Screen {
    Login(Entity<LoginScreen>),
    Main(Entity<MainScreen>),
}

#[derive(Clone)]
enum NavTarget {
    Login,
    Main,
}

#[derive(Clone)]
struct NavigateTo(NavTarget);

impl EventEmitter<NavigateTo> for RootView {}

struct RootView {
    screen: Screen,
    _subscriptions: Vec<Subscription>,
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match self.screen {
            Screen::Login(ref login) => login.clone().into_any_element(),
            Screen::Main(ref main) => main.clone().into_any_element(),
        }
    }
}

fn main() {
    let log_level = if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::EnvFilter::from_default_env()
    } else {
        tracing_subscriber::EnvFilter::new("info")
    };

    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::fmt()
        .with_env_filter(log_level)
        .pretty()
        .init();
    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt::fmt()
        .with_env_filter(log_level)
        .init();

    Application::new().run(|cx| {
        gpui_tokio::init(cx);
        let http = {
            let _guard = Tokio::handle(cx).enter();

            ReqwestClient::new()
        };
        cx.set_http_client(Arc::new(http));

        let user = whoami::username().unwrap_or_else(|_| "nia".to_string());
        let credentials = NavidromeCredentials::load(&user);

        let state = match credentials {
            Ok(creds) => AppState {
                base_url: creds.server.clone(),
                credentials: Some(creds),
            },
            Err(_) => {
                tracing::warn!("failed to load credentials for user {}", user);
                AppState {
                    base_url: String::new(),
                    credentials: None,
                }
            }
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
                    let has_credentials = cx.global::<AppState>().credentials.is_some();

                    if has_credentials {
                        return cx.new(|cx| RootView {
                            screen: Screen::Main(MainScreen::new(cx)),
                            _subscriptions: Vec::new(),
                        });
                    }

                    let login_screen = LoginScreen::new(cx);

                    cx.new(|cx| {
                        let mut subscriptions = vec![];
                        subscriptions.push(cx.subscribe(&login_screen, {
                            let login_screen = login_screen.clone();

                            move |root: &mut RootView, _emitter, event, cx| {
                                root.screen = match event.0 {
                                    NavTarget::Login => Screen::Login(login_screen.clone()),
                                    NavTarget::Main => Screen::Main(MainScreen::new(cx)),
                                };
                                cx.notify();
                            }
                        }));

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
