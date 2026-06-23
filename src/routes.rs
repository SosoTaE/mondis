use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web};
use serde_json::json;
use std::sync::Arc;
use serde_json::Value;

use crate::models::{Model, FilterQuery};
use crate::engine::Engine;



#[get("/health")]
async fn health() -> impl Responder {
    web::Json(json!({"message": "200"}))
}

#[post("/set")]
async fn set(arc_engine: web::Data<Arc<std::sync::RwLock<Engine>>>, data:  web::Json<Model>) -> impl Responder {
    let mut d = data.into_inner();
    let mut engine = arc_engine.write().unwrap();

    engine.set(d.get_key().clone(), d.get_data().clone());

    HttpResponse::Ok().json(json!({"message": "Data received"}))
}


#[post("/find")]
async fn find(arc_engine: web::Data<Arc<std::sync::RwLock<Engine>>>, query: web::Json<FilterQuery>) -> impl Responder {
    let engine = arc_engine.read().unwrap();

    println!("Received GET request for key: {:?}", query.get_key());

    let result = engine.get_filtered(&query);

    HttpResponse::Ok().json(json!({"data": result}))
}


// #[delete("/delete")]
// async fn get_data(arc_engine: web::Data<Arc<std::sync::RwLock<Engine>>>, query: web::Query<GetQuery>) -> impl Responder {
//     let engine = arc_engine.read().unwrap();
    
//     let result = engine.get(query.get_key()).cloned().unwrap_or(Value::Null);

//     HttpResponse::Ok().json(json!({"data": result}))
// }