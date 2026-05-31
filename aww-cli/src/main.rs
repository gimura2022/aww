use std::path::PathBuf;

use clap::Parser;
use directories::BaseDirs;
use miette::miette;

#[derive(Parser)]
#[clap(version)]
enum Args {
    Spawn {
        #[arg(short = 'c', long)]
        config: PathBuf,

        #[arg(short = 'e', long)]
        executable: String
    },
    Refresh {
        #[arg(short = 'c', long)]
        config: PathBuf
    },
    Kill
}

fn is_daemon_running() -> miette::Result<bool> {
    Ok(BaseDirs::new()
        .ok_or(miette!("can't get runtime dir"))?
        .runtime_dir()
        .ok_or(miette!("can't get runtime dir"))?
        .join("aww-ipc")
        .exists())
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
        Args::Spawn { config, executable } => {
            if is_daemon_running()? {
                refesh(config)
            } else {
                spawn(config, executable)
            }
        }
        Args::Refresh { config } => refesh(config),
        Args::Kill => kill()
    }
}
