use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: String,
    pub max_send_length: usize,
    pub max_num_tasks: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    #[serde(rename = "type")]
    pub r#type: String,
    pub task_name: Option<String>,
    pub task_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub response_status: String,
    pub task_id: Option<u32>,
    pub task_name: Option<String>,
    pub message: Option<String>,
}

pub const DEFAULT_PORT: u16 = 8080;
pub const MAX_SEND_LENGTH: usize = 256;
pub const MAX_NUM_TASKS: usize = 5;

impl Config {
    pub fn default() -> Self {
        Config {
            server_addr: "127.0.0.1:8080".to_string(),
            max_send_length: 256,
            max_num_tasks: 5,
        }
    }

    pub fn with_addr(addr: &str) -> Self {
        let mut config = Self::default();
        config.server_addr = addr.to_string();
        config
    }
}

impl Request {
    pub fn new_add(task_name: String) -> Self {
        Request {
            r#type: "add".to_string(),
            task_name: Some(task_name),
            task_id: None,
        }
    }

    pub fn new_get(task_id: u32) -> Self {
        Request {
            r#type: "get".to_string(),
            task_name: None,
            task_id: Some(task_id),
        }
    }

    pub fn new_delete(task_id: u32) -> Self {
        Request {
            r#type: "delete".to_string(),
            task_name: None,
            task_id: Some(task_id),
        }
    }
}
