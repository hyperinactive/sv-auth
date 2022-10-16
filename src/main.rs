mod controllers;
mod routers;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(routers::hello_router()))
        .bind(("127.0.0.1", 4005))?
        .run()
        .await
}
