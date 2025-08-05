use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub priority: Priority,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
    pub next_id: u64,
}

impl Todo {
    pub fn new(title: String, description: Option<String>, priority: Priority) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
            priority,
            due_date: None,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.updated_at = Utc::now();
    }
    
    pub fn mark_incomplete(&mut self) {
        self.completed = false;
        self.updated_at = Utc::now();
    }

    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    pub fn set_due_date(&mut self, due_date: DateTime<Utc>) {
        self.due_date = Some(due_date);
        self.updated_at = Utc::now();
    }
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_todo(&mut self, mut todo: Todo) -> u64 {
        todo.id = self.next_id;
        self.todos.push(todo);
        self.next_id += 1;
        todo.id
    }

    pub fn get_todo(&self, id: u64) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }
    pub fn get_todo_mut(&mut self, id: u64) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == id)
    }

    pub fn remove_todo(&mut self, id: u64) -> bool {
        if let Some(index) = self.todos.iter().position(|todo| todo.id == id) {
            self.todos.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_all_todos(&self) -> &[Todo] {
        &self.todos
    }

    pub fn get_pending_todos(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| !todo.completed).collect()
    }

    pub fn get_completed_todos(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed).collect()
    }

    pub fn get_todos_by_priority(&self, priority: Priority) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| std::mem::discriminant(&todo.priority) == std::mem::discriminant(&priority)).collect()
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}