use std::path::PathBuf;

use directories::BaseDirs;
use miette::miette;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CliCommand {
    RefreshConfig(PathBuf),
    Kill
}

pub fn socket_path() -> miette::Result<PathBuf> {
    Ok(BaseDirs::new()
        .ok_or(miette!("can't get runtime dir"))?
        .runtime_dir()
        .ok_or(miette!("can't get runtime dir"))?
        .join("aww-ipc"))
}
