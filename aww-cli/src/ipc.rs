use std::{
    io::{Read, Write},
    os::unix::net::UnixStream
};

use miette::{IntoDiagnostic, WrapErr};

use aww_ipc::socket_path;

pub fn send(command: &aww_ipc::Command) -> miette::Result<()> {
    let mut stream = UnixStream::connect(socket_path()?)
        .into_diagnostic()
        .wrap_err("can't connect to ipc socket")?;

    stream
        .write_all(
            serde_json::to_string(command)
                .expect("can't serialize ipc message to json")
                .as_bytes()
        )
        .into_diagnostic()
        .wrap_err("can't write ipc message to socket")?;

    let responce = {
        let mut res: Vec<u8> = Vec::new();

        stream.read_to_end(&mut res).into_diagnostic().wrap_err("can't read ipc message from socket")?;

        String::from_utf8(res)
            .into_diagnostic()
            .wrap_err("can't read utf8 string from ipc message")?
    };

    Ok(serde_json::from_str::<aww_ipc::Result<()>>(&responce)
        .into_diagnostic()
        .wrap_err("can't deserialize ipc message to json")??)
}
