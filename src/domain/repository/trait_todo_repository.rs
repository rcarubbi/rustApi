use crate::domain::entities::todo::Todo;
use std::fmt::Error;

pub trait TodoRepositoryTrait: Sync + Send {
    fn create_todo(&self, todo: Todo) -> Result<Todo, Error>;

    fn get_todos(&self) -> Result<Vec<Todo>, Error>;

    fn get_todo_by_id(&self, id: &String) -> Option<Todo>;

    fn update_todo_by_id(&self, id: &str, todo: Todo) -> Option<Todo>;

    fn delete_todo_by_id(&self, id: &str) -> Option<Todo>;
}
