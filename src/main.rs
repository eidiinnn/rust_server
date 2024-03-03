mod db_messages;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use core::panic;
use postgres::{Client, Error, NoTls};

fn main() {
    init_database();
    let _ = server();
}

fn establish_db_connection() -> Result<Client, Error> {
   Client::connect("postgresql://admin:admin@localhost:5432/postgres", NoTls)
}

fn init_database() {
    println!("Starting the database...");
    match establish_db_connection() {
        Ok(mut client) => {
            db_messages::create_message_table(&mut client);
            db_messages::insert_into_messages(&mut client, String::from("hello world"));
        }
        Err(_) => {
            panic!("Error to connection with database")
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/get_last_message")]
async fn get_last_massage() -> impl Responder {
    match establish_db_connection() {
        Ok(mut client) => match db_messages::get_last_message(&mut client) {
            Ok(data) => {
                let json = serde_json::to_string(&data).unwrap();
                HttpResponse::Ok().json(json)
            }
            Err(_) => HttpResponse::InternalServerError().body("Failed to get the last message"),
        },
        Err(error) => {
            println!("Database connection failed: {:?}", error);
            HttpResponse::InternalServerError().body("Database connection failed")
        }
    }
}

#[actix_web::main]
async fn server() -> std::io::Result<()> {
    println!("Starting the web server...");
    HttpServer::new(|| App::new().service(hello).service(get_last_massage))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
