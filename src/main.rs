mod db_messages;

use actix_web::{
    get,
    web::{self, block},
    App, HttpResponse, HttpServer, Responder,
};
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use postgres::{Client, NoTls};
use serde::Deserialize;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref DB_CLIENT: Lazy<Mutex<Client>> = Lazy::new(|| {
        Mutex::new(
            Client::connect("postgresql://admin:admin@localhost:5432/postgres", NoTls).unwrap(),
        )
    });
}

fn main() {
    init_database();
    let _ = server();
}

fn get_connection() -> MutexGuard<'static, Client> {
    let client = DB_CLIENT.lock().unwrap();
    client
}

fn init_database() {
    println!("Starting the database...");
    let mut client = get_connection();
    db_messages::create_message_table(&mut client);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/get_last_message")]
async fn get_last_massage() -> HttpResponse {
    let response = block(|| {
        let mut client = get_connection();
        match db_messages::get_last_message(&mut client) {
            Ok(data) => Ok(serde_json::to_string(&data).unwrap()),
            Err(err) => Err(format!("Database connection failed: {:?}", err)),
        }
    })
    .await;

    match response {
        Ok(json) => HttpResponse::Ok().body(json.unwrap()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct MyData {
   message: String,
}

#[get("/write_message")]
async fn write_message(req: web::Json<MyData>) -> HttpResponse {
    let json = req.into_inner();
    if json.message.is_empty() {
        return HttpResponse::BadRequest().body("Empty message");
    }

    let response = block(|| {
        let mut client = get_connection();
        match db_messages::insert_into_messages(&mut client, json.message) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Database connection failed: {:?}", err)),
        }
    })
    .await;

    match response {
        Ok(_) => HttpResponse::Ok().body(()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn server() -> std::io::Result<()> {
    println!("Starting the web server...");
    HttpServer::new(|| App::new().service(get_last_massage).service(write_message))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
