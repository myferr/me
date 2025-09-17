use actix_web::{HttpResponse, Responder, get};

#[get("/hello-world")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
