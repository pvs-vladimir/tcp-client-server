use std::net::TcpStream;
use std::io::{Read, Write};
use serde_json;
use common::Request;
use crate::args::Cli;
use crate::config::ClientConfig;

pub fn send_request(cli: &Cli, config: &ClientConfig) -> std::io::Result<Vec<u8>> {
    let request = match cli.command.as_str() {
        "add" => Request::new_add(cli.arg.clone().unwrap()),
        "get" => Request::new_get(cli.arg.clone().unwrap().parse().unwrap()),
        "delete" => Request::new_delete(cli.arg.clone().unwrap().parse().unwrap()),
        _ => unreachable!(),
    };

    let mut stream = TcpStream::connect(&config.server_addr)?;

    let request_json = serde_json::to_string(&request)?;
    stream.write_all(request_json.as_bytes())?;
    stream.flush()?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    Ok(buffer)
}
