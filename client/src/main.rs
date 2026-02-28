mod args;
mod request;
mod response;

use clap::Parser;
use args::{Cli, parse_args};
use common::Config;
use request::send_request;
use response::handle_response;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let (command, arg) = match parse_args(cli) {
        Ok(data) => data,
        Err(msg) => {
            eprintln!("{}", msg);
            return Ok(());
        }
    };

    let cli = Cli { command, arg };

    let config = Config::default();

    let response_bytes = send_request(&cli, &config)?;

    handle_response(&response_bytes)?;

    Ok(())
}
