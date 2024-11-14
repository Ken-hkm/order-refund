mod db;
mod handlers;
mod routes;
mod model;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    //set log level
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    std::env::set_var("RUST_LOG", log_level);

    //set db
    let pool = db::get_db_pool().await;

    //set server address and port
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_address = format!("{}{}{}", address,":",port);

    //begin http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::backoffice_routes::init)
            .configure(routes::internal_routes::init)
    })
        .bind(server_address)?
        .run()
        .await
}