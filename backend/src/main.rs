use actix_web::{App, HttpServer};

mod routes;
use crate::routes::hello_world::hello_world;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running at localhost:3000");
    HttpServer::new(|| App::new().service(hello_world))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
