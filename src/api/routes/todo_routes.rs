use crate::api::todo_controller;
use actix_web::web;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route("/",  web::get().to(todo_controller::get_todos))
            .route("/{id}",  web::get().to(todo_controller::get_todo_by_id))
            .route("/",  web::post().to(todo_controller::create_todo))
            .route("/{id}",  web::put().to(todo_controller::update_todo_by_id))
            .route("/{id}",  web::delete().to(todo_controller::delete_todo_by_id))
    );
}
