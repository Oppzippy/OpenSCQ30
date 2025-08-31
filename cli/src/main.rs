mod cli;
mod fmt;
mod parse;

use std::{path::PathBuf, process::ExitCode};

use anyhow::anyhow;
use dirs::config_dir;
use openscq30_lib::OpenSCQ30Session;
use tracing::Level;

#[tokio::main]
async fn main() -> ExitCode {
    let matches = cli::build().get_matches();
    if matches.get_flag("verbose") {
        tracing_subscriber::fmt()
            .with_file(true)
            .with_line_number(true)
            .with_target(true)
            .with_max_level(Level::WARN)
            .with_writer(std::io::stderr)
            .pretty()
            .init();
    }
    if let Err(err) = cli::handle(&matches).await {
        if matches.get_flag("debug-errors") {
            eprintln!("Error: {err:?}");
        } else {
            // display anyhow context chain on one line
            eprintln!("Error: {err:#}");
        }
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

pub async fn openscq30_session() -> anyhow::Result<OpenSCQ30Session> {
    let db_path = match std::env::var_os("OPENSCQ30_DATABASE_PATH") {
        Some(path) => PathBuf::from(path),
        None => config_dir()
            .ok_or_else(|| anyhow!("failed to find config dir"))?
            .join("openscq30")
            .join("database.sqlite"),
    };
    OpenSCQ30Session::new(db_path).await.map_err(Into::into)
}
