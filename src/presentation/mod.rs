use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::application; // application 모듈을 가져옵니다.
use serde::Deserialize;
use crate::infrastructure::database::establish_connection; // 데이터베이스 연결을 설정

#[derive(serde::Deserialize)]
struct TestData {
    name: String,
    email: String,
}

async fn create_test(data: web::Json<TestData>) -> impl Responder {
    let mut conn = establish_connection(); // mutable reference로 설정
    let result = application::create_test(&mut conn, &data.name, &data.email); // &mut conn 사용
    match result {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

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
            .route("/create_test", web::post().to(create_test)) // POST 경로 추가
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
