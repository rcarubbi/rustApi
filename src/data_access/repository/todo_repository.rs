use chrono::prelude::*;
use std::fmt::Error;
 
 

use crate::domain::{entities::todo::Todo, repository::trait_todo_repository::TodoRepositoryTrait};

pub struct TodoRepository {
    
}

impl TodoRepositoryTrait for TodoRepository {
    fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        Ok(Todo {
            id: None,
            created_at: None,
            updated_at: None,
            title: String::from(""),
            description: None
        })
    }

    fn get_todos(&self) -> Result<Vec<Todo>, Error> {
        Ok(Vec::new())
    }

    fn get_todo_by_id(&self, id: &String) -> Option<Todo> {
        None
    }

    fn update_todo_by_id(&self, id: &str, todo: Todo) -> Option<Todo> {
        None
    }

    fn delete_todo_by_id(&self, id: &str) -> Option<Todo> {
        None
    }
}

impl TodoRepository {
 
    pub fn new() -> Self  {
        TodoRepository {}
    }
  
}
