use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(version)]
enum Args {
    Spawn {
        #[arg(short = 'c', long)]
        config: PathBuf,

        #[arg(short = 'e', long)]
        executable: String,
    },
    Refesh {
        #[arg(short = 'c', long)]
        config: PathBuf,
    },
    Kill,
}

fn is_daemon_running() -> bool {
    todo!()
}

fn spawn(_config: PathBuf, _executable: String) -> miette::Result<()> {
    todo!()
}

fn refesh(_config: PathBuf) -> miette::Result<()> {
    todo!()
}

fn kill() -> miette::Result<()> {
    todo!()
}

fn main() -> miette::Result<()> {
    match Args::parse() {
        Args::Spawn { config, executable } => if is_daemon_running() { refesh(config) } else { spawn(config, executable) },
        Args::Refesh { config } => refesh(config),
        Args::Kill => kill(),
    }?;

    Ok(())
}
