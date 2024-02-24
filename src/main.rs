mod db_messages;
use core::panic;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use postgres::{Client, NoTls};

fn main() {
    database();
    let _ = server();
}

fn database() {
    println!("Starting the database...");
    match Client::connect("postgresql://admin:admin@localhost:5432/postgres", NoTls) {
        Ok(mut client) => {
            db_messages::create_message_table(&mut client);
            db_messages::insert_into_messages(&mut client, String::from("it's a message"));
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

#[actix_web::main]
async fn server() -> std::io::Result<()> {
    println!("Starting the web server...");
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
