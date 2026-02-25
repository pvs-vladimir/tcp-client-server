use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use common::Request;
use crate::handlers::handle_request;
use crate::task_store::TaskStore;

mod task_store;
mod handlers;

fn handle_client(mut stream: TcpStream, store: &mut TaskStore) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    if buffer.is_empty() {
        return Ok(());
    }

    let request: Request = serde_json::from_slice(&buffer).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
    })?;

    let response = handle_request(request, store);
    let response_json = serde_json::to_string(&response)?;
    stream.write_all(response_json.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Сервер запущен на 127.0.0.1:8080");

    let mut store = TaskStore::new(5);

    loop {
        let (stream, addr) = listener.accept()?;
        println!("Подключение от {}", addr);

        if let Err(e) = handle_client(stream, &mut store) {
            eprintln!("Ошибка обработки клиента: {}", e);
        }
    }
}
