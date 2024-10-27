mod presentation;
mod application;
mod domain;
mod infrastructure;
mod schema; // schema 모듈을 선언

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    presentation::run_server().await
}