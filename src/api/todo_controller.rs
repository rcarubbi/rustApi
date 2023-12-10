use crate::domain::{entities::todo::Todo, repository::trait_todo_repository::TodoRepositoryTrait};
use std::sync::Arc;

use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};

pub async fn create_todo(
    new_todo: Json<Todo>,
    repository: Data<Arc<dyn TodoRepositoryTrait>>,
) -> HttpResponse {
    let todo = repository.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_todos(repository: Data<Arc<dyn TodoRepositoryTrait>>) -> HttpResponse {
    let todos = repository.get_todos();
    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_todo_by_id(
    id: web::Path<String>,
    repository: Data<Arc<dyn TodoRepositoryTrait>>,
) -> HttpResponse {
    let todo = repository.get_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub async fn update_todo_by_id(
    id: web::Path<String>,
    updated_todo: web::Json<Todo>,
    repository: Data<Arc<dyn TodoRepositoryTrait>>,
) -> HttpResponse {
    let todo = repository.update_todo_by_id(&id, updated_todo.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub async fn delete_todo_by_id(
    id: web::Path<String>,
    repository: Data<Arc<dyn TodoRepositoryTrait>>,
) -> HttpResponse {
    let todo = repository.delete_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}
