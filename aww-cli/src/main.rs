use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[clap(version)]
enum Args {
    Spawn {
        #[arg(short = 'c', long)]
        config: PathBuf,
    },
    Kill,
}

fn spawn(_config: PathBuf) -> miette::Result<()> {
    todo!()
}

fn kill() -> miette::Result<()> {
    todo!()
}

fn main() -> miette::Result<()> {
    match Args::parse() {
        Args::Spawn { config } => spawn(config),
        Args::Kill => kill(),
    }?;

    Ok(())
}
