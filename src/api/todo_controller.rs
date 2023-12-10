use crate::{models::todo::Todo, repository::todo_repository::TodoRepository};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};

pub async fn create_todo(repository: Data<TodoRepository>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = repository.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_todos(repository: Data<TodoRepository>) -> HttpResponse {
    let todos = repository.get_todos();
    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_todo_by_id(repository: web::Data<TodoRepository>, id: web::Path<String>) -> HttpResponse {
    let todo = repository.get_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub async fn update_todo_by_id(
    repository: web::Data<TodoRepository>,
    id: web::Path<String>,
    updated_todo: web::Json<Todo>,
) -> HttpResponse {
    let todo = repository.update_todo_by_id(&id, updated_todo.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub async fn delete_todo_by_id(repository: web::Data<TodoRepository>, id: web::Path<String>) -> HttpResponse {
    let todo = repository.delete_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

