use std::sync::Mutex;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

use crate::blockchain::Blockchain;
use crate::routes::configure_routes;

mod blockchain;
mod routes;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = web::Data::new(Mutex::new(Blockchain::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(blockchain.clone())
            .configure(configure_routes)
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
