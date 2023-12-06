use crate::{models::todo::Todo, repository::todo_repository::TodoRepository};
use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json},
    HttpResponse,
};

#[post("/todos")]
pub async fn create_todo(repository: Data<TodoRepository>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = repository.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(repository: Data<TodoRepository>) -> HttpResponse {
    let todos = repository.get_todos();
    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(repository: web::Data<TodoRepository>, id: web::Path<String>) -> HttpResponse {
    let todo = repository.get_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/todos/{id}")]
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

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(repository: web::Data<TodoRepository>, id: web::Path<String>) -> HttpResponse {
    let todo = repository.delete_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_todo)
            .service(get_todos)
            .service(get_todo_by_id)
            .service(update_todo_by_id)
            .service(delete_todo_by_id),
    );
}
