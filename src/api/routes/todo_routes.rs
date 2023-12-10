use crate::api::todo_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(
                web::resource("")
                    .route(web::get().to(todo_controller::get_todos))
                    .route(web::post().to(todo_controller::create_todo)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(todo_controller::get_todo_by_id))
                    .route(web::put().to(todo_controller::update_todo_by_id))
                    .route(web::delete().to(todo_controller::delete_todo_by_id)),
            ),
    );
}
