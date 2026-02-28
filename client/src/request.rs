use std::net::TcpStream;
use std::io::{Read, Write};
use serde_json;
use common::{Config, MAX_SEND_LENGTH, Request};
use crate::args::Cli;

pub fn send_request(cli: &Cli, config: &Config) -> std::io::Result<Vec<u8>> {
    let request = match cli.command.as_str() {
        "add" => Request::new_add(cli.name.clone().unwrap()),
        "get" => Request::new_get(cli.id.unwrap()),
        "delete" => Request::new_delete(cli.id.unwrap()),
        _ => unreachable!(),
    };

    let mut stream = TcpStream::connect(&config.server_addr)?;

    let request_json = serde_json::to_string(&request)?;
    stream.write_all(request_json.as_bytes())?;
    stream.flush()?;

    let mut buffer = [0; MAX_SEND_LENGTH];
    let bytes_read = stream.read(&mut buffer)?;
    let response_slice = &buffer[..bytes_read];

    Ok(response_slice.to_vec())
}
