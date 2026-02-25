use common::{Request, Response};
use crate::task_store::TaskStore;

pub fn handle_add(request: Request, store: &mut TaskStore) -> Response {
    match request.task_name {
        Some(name) => match store.add_task(name) {
            Ok(id) => Response {
                response_status: "success".to_string(),
                task_id: Some(id),
                task_name: None,
                message: None,
            },
            Err(msg) => Response {
                response_status: "fail".to_string(),
                task_id: None,
                task_name: None,
                message: Some(msg),
            },
        },
        None => Response {
            response_status: "fail".to_string(),
            task_id: None,
            task_name: None,
            message: Some("missing task_name".to_string()),
        },
    }
}

pub fn handle_get(request: Request, store: &TaskStore) -> Response {
    match request.task_id {
        Some(id) => match store.get_task(id) {
            Some(name) => Response {
                response_status: "success".to_string(),
                task_id: Some(id),
                task_name: Some(name.clone()),
                message: None,
            },
            None => Response {
                response_status: "fail".to_string(),
                task_id: None,
                task_name: None,
                message: Some("task not found".to_string()),
            },
        },
        None => Response {
            response_status: "fail".to_string(),
            task_id: None,
            task_name: None,
            message: Some("missing task_id".to_string()),
        },
    }
}

pub fn handle_delete(request: Request, store: &mut TaskStore) -> Response {
    match request.task_id {
        Some(id) => match store.delete_task(id) {
            Some(name) => Response {
                response_status: "success".to_string(),
                task_id: Some(id),
                task_name: Some(name),
                message: None,
            },
            None => Response {
                response_status: "fail".to_string(),
                task_id: None,
                task_name: None,
                message: Some("task not found".to_string()),
            },
        },
        None => Response {
            response_status: "fail".to_string(),
            task_id: None,
            task_name: None,
            message: Some("missing task_id".to_string()),
        },
    }
}

pub fn handle_request(request: Request, store: &mut TaskStore) -> Response {
    match request.r#type.as_str() {
        "add" => handle_add(request, store),
        "get" => handle_get(request, store),
        "delete" => handle_delete(request, store),
        _ => Response {
            response_status: "fail".to_string(),
            task_id: None,
            task_name: None,
            message: Some("unknown request type".to_string()),
        },
    }
}
