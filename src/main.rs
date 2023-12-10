use actix_web::{self, get, web, App, HttpResponse, HttpServer, Responder, Result};
use domain::repository::trait_todo_repository::TodoRepositoryTrait;
use serde::Serialize;
use std::{sync::Arc, env};
mod api;
mod data_access;
mod domain;

use data_access::repository::todo_repository::TodoRepository;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_repository: Arc<dyn TodoRepositoryTrait> = Arc::new(TodoRepository::new());
    HttpServer::new(move || {

        App::new()
            .app_data(web::Data::new(Arc::clone(&todo_repository)))
            .configure(|app| {
                app.service(web::scope("/api").configure(api::routes::config));
            })
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
