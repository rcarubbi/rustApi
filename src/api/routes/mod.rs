
use actix_web::web;
pub mod todo_routes;


pub fn config(cfg: &mut web::ServiceConfig) {
    todo_routes::config(cfg);
}