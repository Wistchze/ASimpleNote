mod server_config;
mod notes;
mod routes;
mod handler;
mod server_error;
mod utils;

use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use notes::Note;
use server_config::ServerConfig;

struct AppState {
    pub notes: Mutex<Vec<Note>>,
    // pub server_cfg: ServerConfig
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init Server
    env_logger::init();
    let server_cfg = ServerConfig::new();
    let addr = format!("{}:{}", &server_cfg.host, &server_cfg.port);

    // Data
    let data = web::Data::new(AppState {
        notes: Mutex::new(Vec::new()),
        // server_cfg
    });

    // Run Server
    println!("Server is running at addr: {}", &addr);

    HttpServer::new(move ||{
        App::new()
            .wrap(Cors::default())
            .wrap(Logger::default())
            .app_data(data.clone())
            .configure(routes::init)
    })
        .bind(addr)?
        .run()
        .await
}
