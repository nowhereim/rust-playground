use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::application; // application 모듈을 가져옵니다.

async fn hello() -> impl Responder {
    let message = application::get_hello_message();
    HttpResponse::Ok().body(message)
}

async fn hi() -> impl Responder {
    let message = application::get_hi_message();
    HttpResponse::Ok().body(message)
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(hello))
            .route("/hi", web::get().to(hi))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}