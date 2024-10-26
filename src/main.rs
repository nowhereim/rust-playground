mod presentation;
mod application;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    presentation::run_server().await
}