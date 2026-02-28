mod args;
mod request;
mod response;

use clap::Parser;
use args::Cli;
use common::Config;
use request::send_request;
use response::handle_response;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    if let Err(msg) = cli.validate() {
        eprintln!("{}", msg);
        return Ok(());
    }

    let config = Config::default();
    let response_bytes = send_request(&cli, &config)?;
    handle_response(&response_bytes)?;

    Ok(())
}
