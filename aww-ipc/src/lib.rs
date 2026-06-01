use std::{env, path::PathBuf};

use directories::BaseDirs;
use miette::{IntoDiagnostic, WrapErr, miette};
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
        .join(format!(
            "aww-{}-ipc",
            env::var("WAYLAND_DISPLAY")
                .into_diagnostic()
                .wrap_err("can't get WAYLAND_DISPLAY")?
        )))
}
