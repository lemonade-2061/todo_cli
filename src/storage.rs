use crate::models::TodoList;
use serde_json;
use std::fs;
use std::path::Path;

pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn load(&self) -> Result<TodoList, Box<dyn std::error::Error>> {
        if Path::new(&self.file_path).exists() {
            let content = fs::read_to_string(&self.file_path)?;
            let todo_list: TodoList = serde_json::from_str(&content)?;
            Ok(todo_list)
        } else {
            Ok(TodoList::default())
        }
    }

    pub fn save(&self, todo_list: &TodoList) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(todo_list)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
}