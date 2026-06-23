mod engine;
mod models;
mod routes;

use actix_web::{App, HttpServer, web, middleware::Logger};
use log::{info, warn, error};
use std::sync::{RwLock, Arc};
use std::thread;

use crate::engine::Engine;
use crate::routes::{health, set, find};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arc_engine = Arc::new(RwLock::new(Engine::new()));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

   info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a \"%r\" %s %T"))
            .service(health)
            .app_data(web::JsonConfig::default().limit(52_428_800))
            .app_data(web::Data::new(arc_engine.clone()))
            .service(set)
            .service(find)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
