mod cli;
mod fmt;
mod parse;

use std::{path::PathBuf, process::ExitCode};

use anyhow::anyhow;
use clap::ArgMatches;
use dirs::config_dir;
use openscq30_lib::OpenSCQ30Session;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[cfg(not(target_os = "macos"))]
#[tokio::main]
async fn main() -> ExitCode {
    run().await
}

// IOBluetooth delivers delegate callbacks (RFCOMM data, channel open/close) through the
// process's main thread run loop regardless of which thread issued the call, so tokio can't be
// left owning the real OS main thread here the way #[tokio::main] normally would: nothing would
// ever pump a run loop on it, and every callback would silently never arrive.
#[cfg(target_os = "macos")]
fn main() -> ExitCode {
    let (exit_code_sender, exit_code_receiver) = std::sync::mpsc::channel();
    let join_handle = std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("failed to start tokio runtime");
        let _ = exit_code_sender.send(runtime.block_on(run()));
    });

    loop {
        match exit_code_receiver.try_recv() {
            Ok(code) => return code,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                // The worker thread panicked before sending a result (the panic hook already
                // printed it). Match the exit code Rust's default runtime uses when the real
                // main thread panics, since the actual work here runs on a worker thread instead.
                return match join_handle.join() {
                    Ok(()) => ExitCode::FAILURE,
                    Err(_) => ExitCode::from(101),
                };
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
        }
        let result = unsafe {
            objc2_core_foundation::CFRunLoop::run_in_mode(
                objc2_core_foundation::kCFRunLoopDefaultMode,
                0.1,
                false,
            )
        };
        // run_in_mode returns immediately, not after the given timeout, whenever nothing is
        // scheduled on the run loop yet: sleep here so that doesn't turn into a busy spin.
        if result == objc2_core_foundation::CFRunLoopRunResult::Finished {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

async fn run() -> ExitCode {
    let matches = cli::build().get_matches();

    if let Err(err) = initialize_logging(&matches) {
        eprintln!("Logging error: {err:?}");
        return ExitCode::FAILURE;
    }

    if let Err(err) = cli::handle(&matches).await {
        if matches.get_count("verbose") > 0 || matches.get_flag("debug-errors") {
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

fn initialize_logging(matches: &ArgMatches) -> anyhow::Result<()> {
    let log_level_filter = match matches.get_count("verbose") {
        0 => None,
        1 => Some(LevelFilter::WARN),
        2 => Some(LevelFilter::INFO),
        3 => Some(LevelFilter::DEBUG),
        _ => Some(LevelFilter::TRACE),
    };

    if let Some(log_level_filter) = log_level_filter {
        tracing_subscriber::fmt()
            .with_file(true)
            .with_line_number(true)
            .with_target(true)
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(log_level_filter.into())
                    .from_env()?,
            )
            .with_writer(std::io::stderr)
            .pretty()
            .init();
    }
    Ok(())
}
