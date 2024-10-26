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

async fn calculation() -> impl Responder {
    let result = application::get_calculation_result();
    HttpResponse::Ok()
    .content_type("text/plain; charset=utf-8")
    .body(result)
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(hello))
            .route("/hi", web::get().to(hi))
            .route("/calculation", web::get().to(calculation))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
