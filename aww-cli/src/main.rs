use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    process::Command
};

use clap::Parser;
use directories::BaseDirs;
use miette::{IntoDiagnostic, WrapErr, miette};

use aww_ipc::socket_path;

mod ipc;

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
    Status,
    Kill
}

fn daemon_running() -> miette::Result<bool> {
    Ok(socket_path()?.exists())
}

fn config_path() -> miette::Result<PathBuf> {
    Ok(BaseDirs::new()
        .ok_or(miette!("can't get config dir"))?
        .preference_dir()
        .join("aww"))
}

fn spawn(config: Cow<Path>, executable: &str) -> miette::Result<()> {
    Command::new(executable)
        .spawn()
        .into_diagnostic()
        .wrap_err("can't run daemon process")?;
    refesh(config)
}

fn refesh(config: Cow<Path>) -> miette::Result<()> {
    ipc::send(&aww_ipc::Command::RefreshConfig(config.to_path_buf()))
}

fn status() -> miette::Result<()> {
    if daemon_running()? {
        println!("daemon is running");
    } else {
        println!("daemon is not running");
    }

    Ok(())
}

fn kill() -> miette::Result<()> {
    ipc::send(&aww_ipc::Command::Kill)
}

fn main() -> miette::Result<()> {
    match Args::parse() {
        Args::Spawn { config, executable } => {
            if daemon_running()? {
                refesh(config.unwrap_or(config_path()?).into())
            } else {
                spawn(config.unwrap_or(config_path()?).into(), &executable)
            }
        }
        Args::Refresh { config } => refesh(config.unwrap_or(config_path()?).into()),
        Args::Status => status(),
        Args::Kill => kill()
    }
}
