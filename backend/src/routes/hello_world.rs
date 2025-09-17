use actix_web::{get, HttpResponse, Responder};

#[get("/hello-world")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
