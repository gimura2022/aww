use std::{io::Write, os::unix::net::UnixStream};

use miette::{IntoDiagnostic, WrapErr};

use aww_ipc::{CliCommand, socket_path};

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
