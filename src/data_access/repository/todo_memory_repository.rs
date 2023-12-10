use std::fmt::Error;
use std::sync::{Arc, Mutex};
use chrono::prelude::*;

use crate::domain::{entities::todo::Todo, repository::trait_todo_repository::TodoRepositoryTrait};

pub struct TodoMemoryRepository {
    todos: Arc<Mutex<Vec<Todo>>>,
}

impl TodoRepositoryTrait for TodoMemoryRepository {
    
    fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let mut todos = self.todos.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let todo = Todo {
            id: Some(id),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..todo
        };
        todos.push(todo.clone());
        Ok(todo)
    }

    fn get_todos(&self) -> Result<Vec<Todo>, Error> {
        Ok(self.todos.lock().unwrap().clone())
    }

    fn get_todo_by_id(&self, id: &String) -> Option<Todo> {
        let todos = self.todos.lock().unwrap();
        todos.iter().find(|todo| todo.id == Some(id.to_string())).cloned()
    }

    fn update_todo_by_id(&self, id: &str, todo: Todo) -> Option<Todo> {
        let mut todos = self.todos.lock().unwrap();
        let updated_at = Utc::now();
        let index = todos.iter().position(|todo| todo.id == Some(id.to_string()))?;
        let todo = Todo {
            id: Some(id.to_string()),
            updated_at: Some(updated_at),
            created_at: todos[index].created_at,
            ..todo
        };
        todos[index] = todo.clone();
        Some(todo)
    }

    fn delete_todo_by_id(&self, id: &str) -> Option<Todo> {
        let mut todos = self.todos.lock().unwrap();
        let index = todos.iter().position(|todo| todo.id == Some(id.to_string()))?;
        Some(todos.remove(index))
    }

}
 
impl TodoMemoryRepository {
    pub fn new() -> Self {
        let todos = Arc::new(Mutex::new(vec![]));
        TodoMemoryRepository { todos }
    }
}