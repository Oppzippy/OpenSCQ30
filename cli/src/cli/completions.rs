use std::io;

use anyhow::anyhow;
use clap::ArgMatches;

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let mut command = crate::cli::build();
    let command_name = command.get_name().to_string();
    clap_complete::generate(
        matches
            .get_one::<clap_complete::Shell>("shell")
            .copied()
            .ok_or_else(|| anyhow!("no shell specified"))?,
        &mut command,
        command_name,
        &mut io::stdout(),
    );
    Ok(())
}
