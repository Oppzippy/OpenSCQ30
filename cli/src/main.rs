mod cli;
mod fmt;

use std::path::PathBuf;

use anyhow::anyhow;
use dirs::config_dir;
use openscq30_lib::api::OpenSCQ30Session;
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = cli::build().get_matches();
    cli::handle(&matches).await?;
    Ok(())
}

pub async fn openscq30_session() -> anyhow::Result<OpenSCQ30Session> {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_max_level(Level::WARN)
        .with_writer(std::io::stderr)
        .pretty()
        .init();

    let db_path = match std::env::var_os("OPENSCQ30_DATABASE_PATH") {
        Some(path) => PathBuf::from(path),
        None => config_dir()
            .ok_or_else(|| anyhow!("failed to find config dir"))?
            .join("openscq30")
            .join("database.sqlite"),
    };
    OpenSCQ30Session::new(db_path).await.map_err(Into::into)
}
