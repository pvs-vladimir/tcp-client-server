use std::collections::HashMap;

pub struct TaskStore {
    tasks: HashMap<u32, String>,
    next_id: u32,
    capacity: usize
}

impl TaskStore {
    pub fn new(capacity: usize) -> Self {
        TaskStore {
            tasks: HashMap::new(),
            next_id: 1,
            capacity: capacity,
        }
    }

    pub fn add_task(&mut self, name: String) -> Result<u32, String> {
        if self.tasks.len() >= self.capacity {
            return Err("too many tasks".to_string());
        }
        let id = self.next_id;
        self.tasks.insert(id, name);
        self.next_id += 1;
        Ok(id)
    }

    pub fn get_task(&self, id: u32) -> Option<&String> {
        self.tasks.get(&id)
    }

    pub fn delete_task(&mut self, id: u32) -> Option<String> {
        self.tasks.remove(&id)
    }
}
