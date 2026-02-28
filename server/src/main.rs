use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use common::{Config, MAX_SEND_LENGTH, Request, Response};
use crate::task_store::TaskStore;
use crate::handlers::handle_request;

mod task_store;
mod handlers;

fn handle_client(mut stream: TcpStream, store: &mut TaskStore) -> std::io::Result<()> {
    let mut buffer = [0; MAX_SEND_LENGTH];
    let bytes_read = stream.read(&mut buffer)?;

    if buffer.is_empty() {
        return Ok(());
    }
    
    let request_slice = &buffer[..bytes_read];
    let request: Request = serde_json::from_slice(&request_slice).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
    })?;

    let response: Response = handle_request(request, store);
    let response_json = serde_json::to_string(&response)?;
    stream.write_all(response_json.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let config = Config::default();

    let listener = TcpListener::bind(&config.server_addr)?;
    println!("Сервер запущен на {}", config.server_addr);

    let mut store = TaskStore::new(config.max_num_tasks);

    loop {
        let (stream, addr) = listener.accept()?;
        println!("Подключение от {}", addr);

        if let Err(e) = handle_client(stream, &mut store) {
            eprintln!("Ошибка обработки клиента: {}", e);
        }
    }
}
