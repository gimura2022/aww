use std::{env, path::PathBuf};

use directories::BaseDirs;
use miette::{Diagnostic, IntoDiagnostic, WrapErr, miette};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub enum Command {
    RefreshConfig(PathBuf),
    Kill
}

#[derive(Serialize, Deserialize, Error, Diagnostic, Debug)]
pub enum Error {}

pub type Result<T> = core::result::Result<T, Error>;

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
