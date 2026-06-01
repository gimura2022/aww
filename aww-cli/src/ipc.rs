use std::{io::Write, os::unix::net::UnixStream, path::PathBuf};

use directories::BaseDirs;
use miette::{IntoDiagnostic, WrapErr, miette};

use aww_ipc::CliCommand;

pub fn socket_path() -> miette::Result<PathBuf> {
    Ok(BaseDirs::new()
        .ok_or(miette!("can't get runtime dir"))?
        .runtime_dir()
        .ok_or(miette!("can't get runtime dir"))?
        .join("aww-ipc"))
}

pub fn send(command: &CliCommand) -> miette::Result<()> {
    UnixStream::connect(socket_path()?)
        .into_diagnostic()
        .wrap_err("can't connect to ipc socket")?
        .write_all(
            serde_json::to_string(command)
                .expect("can't serialize ipc message to json")
                .as_bytes()
        )
        .into_diagnostic()
        .wrap_err("can't write ipc message to socket")
}
