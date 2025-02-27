mod response;
mod model;
mod handler;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, http, web};
use model::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let todo_db = AppState::init();
    let app_data = web::Data::new(todo_db);

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                http::header::CONTENT_TYPE,
                http::header::AUTHORIZATION,
                http::header::ACCEPT
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
