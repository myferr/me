use actix_cors::Cors;
use actix_web::{App, HttpServer};

mod routes;
use crate::routes::blog::{get_post, get_posts};
use crate::routes::hello_world::hello_world;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running at localhost:2025");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(hello_world)
            .service(get_post)
            .service(get_posts)
    })
    .bind(("0.0.0.0", 2025))?
    .run()
    .await
}
