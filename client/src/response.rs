use serde_json;
use std::io;
use common::Response;

pub fn handle_response(response_bytes: &[u8]) -> io::Result<()> {
    if response_bytes.is_empty() {
        eprintln!("Сервер не ответил");
        return Ok(());
    }

    let response: Response = serde_json::from_slice(response_bytes).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, e.to_string())
    })?;

    match response.response_status.as_str() {
        "success" => {
            println!("Операция выполнена успешно");
            if let Some(id) = response.task_id {
                println!("ID задачи: {}", id);
            }
            if let Some(ref name) = response.task_name {
                println!("Название: {}", name);
            }
        }
        "fail" => {
            if let Some(ref msg) = response.message {
                eprintln!("Ошибка: {}", msg);
            } else {
                eprintln!("Операция завершилась с ошибкой");
            }
        }
        other => {
            eprintln!("Неизвестный статус ответа: {}", other);
        }
    }

    Ok(())
}
