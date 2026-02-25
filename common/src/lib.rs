use serde::{Deserialize, Serialize};

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
