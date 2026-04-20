use std::process::ExitCode;

use clap::{Parser, Subcommand};
use nia_navidrome::auth::NavidromeCredentials;

struct Detect;

#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(global = true, long)]
    debug: bool,

    #[arg(global = false, long)]
    foreground: bool,

    #[arg(global = true, long)]
    version: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    Login {
        server: String,
        username: String,
        password: String,
    },

    NowPlaying {
        server: String,
    },
}

trait InstalledApp {
    fn version_string(&self) -> String;
    fn launch(&self) -> anyhow::Result<()>;
    fn run_foreground(&self) -> anyhow::Result<()>;
}

fn main() -> ExitCode {
    match inner_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            tracing::error!("{err:?}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod linux {
    use std::env;
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};
    use std::process::{self, ExitStatus};

    use anyhow::{Context, anyhow};
    use fork::Fork;
    use tokio::io;

    use crate::{Detect, InstalledApp};

    struct App(PathBuf);

    impl Detect {
        pub fn detect(path: Option<&Path>) -> anyhow::Result<impl InstalledApp> {
            let path = if let Some(path) = path {
                path.to_path_buf().canonicalize()?
            } else {
                let cli = env::current_exe()?;
                let dir = cli.parent().context("no parent path for cli")?;

                let possible_locations = ["../libexec/nia-music", "../lib/nia/nia-music", "./nia"];
                possible_locations
                    .iter()
                    .find_map(|p| dir.join(p).canonicalize().ok().filter(|path| path != &cli))
                    .with_context(|| {
                        format!("could not find any of: {}", possible_locations.join(", "))
                    })?
            };

            Ok(App(path))
        }
    }

    impl InstalledApp for App {
        fn version_string(&self) -> String {
            "nia v2026.4-alpha".to_string()
        }

        fn launch(&self) -> anyhow::Result<()> {
            self.boot_background()?;

            Ok(())
        }

        fn run_foreground(&self) -> anyhow::Result<()> {
            self.run_foreground()?;

            Ok(())
        }
    }

    impl App {
        fn boot_background(&self) -> anyhow::Result<()> {
            let path = &self.0;

            match fork::fork() {
                Ok(Fork::Parent(_)) => Ok(()),
                Ok(Fork::Child) => {
                    if fork::setsid().is_err() {
                        eprintln!(
                            "failed to set session id: {}",
                            std::io::Error::last_os_error()
                        );
                        process::exit(1)
                    }

                    if fork::close_fd().is_err() {
                        eprintln!(
                            "failed to close file descriptors: {}",
                            std::io::Error::last_os_error()
                        );
                        process::exit(1)
                    }

                    let error = exec::execvp::<PathBuf, Vec<OsString>>(path.clone(), vec![]);
                    eprintln!("failed to execvp: {}", error);
                    process::exit(1)
                }
                Err(_) => Err(anyhow!(io::Error::last_os_error())),
            }
        }

        fn run_foreground(&self) -> io::Result<ExitStatus> {
            let mut cmd = std::process::Command::new(self.0.clone());

            cmd.status()
        }
    }
}

#[tokio::main]
async fn inner_main() -> anyhow::Result<()> {
    let args = Args::parse();

    let log_level = if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::EnvFilter::from_default_env()
    } else if args.debug {
        tracing_subscriber::EnvFilter::new("nia_cli=debug")
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

    let app = Detect::detect(None)?;

    if args.version {
        println!("{}", app.version_string());

        return Ok(());
    }

    match args.command {
        None => {
            if args.foreground {
                app.run_foreground()?;
            } else {
                app.launch()?;
            }
        }

        Some(Command::Login {
            server,
            username,
            password,
        }) => {
            // let mut navidrome = nia_navidrome::NavidromeClient::new(server);
            // navidrome.login(username, password).await?;
        }

        Some(Command::NowPlaying { server }) => {
            // let creds = NavidromeCredentials::load(&server).await?;
            // let navidrome = nia_navidrome::NavidromeClient::with_credentials(server,
            // creds.clone());

            // tracing::info!("creds: {:?}", creds);

            // let resp = navidrome.get_now_playing().await?;

            // tracing::info!("received resp: {:?}", resp);

            // tracing::info!(
            // "now playing: {:?}",
            // resp.inner_subsonic_response
            // .body
            // .now_playing
            // .entry
            // .first()
            // .as_ref()
            // .map(|f| f.title.clone())
            // );
        }
    }

    Ok(())
}
