mod presentation;
mod application;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    presentation::run_server().await
}