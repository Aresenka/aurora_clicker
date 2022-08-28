mod api;
mod model;
mod repository;

use std::fs::File;
use std::io::Result;
use actix_web::{HttpServer, App, web, middleware::Logger, Responder};
use actix_files::{Files, NamedFile};
use serde;

async fn index() -> impl Responder {
    NamedFile::open_async("../frontend/index.html").await.unwrap()
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(web::resource("/").to(index))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}