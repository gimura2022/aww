use std::path::PathBuf;

use clap::Parser;
use directories::BaseDirs;
use miette::miette;

#[derive(Parser)]
#[clap(version)]
enum Args {
    Spawn {
        #[arg(short = 'c', long)]
        config: Option<PathBuf>,

        #[arg(short = 'e', long, default_value = "aww-daemon")]
        executable: String
    },
    Refresh {
        #[arg(short = 'c', long)]
        config: Option<PathBuf>
    },
    Kill
}

fn daemon_running() -> miette::Result<bool> {
    Ok(BaseDirs::new()
        .ok_or(miette!("can't get runtime dir"))?
        .runtime_dir()
        .ok_or(miette!("can't get runtime dir"))?
        .join("aww-ipc")
        .exists())
}

fn config_path() -> miette::Result<PathBuf> {
    Ok(BaseDirs::new()
        .ok_or(miette!("can't get config dir"))?
        .preference_dir()
        .join("aww"))
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
            if daemon_running()? {
                refesh(config.unwrap_or(config_path()?))
            } else {
                spawn(config.unwrap_or(config_path()?), executable)
            }
        }
        Args::Refresh { config } => refesh(config.unwrap_or(config_path()?)),
        Args::Kill => kill()
    }
}
