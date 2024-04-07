mod api_endpoints;
mod db;
mod db_messages;
use actix_web::{App, HttpServer};

fn main() {
    init_database();
    let _ = server();
}

fn init_database() {
    println!("Starting the database...");
    let mut client = db::get_connection();
    db_messages::create_message_table(&mut client);
}

#[actix_web::main]
async fn server() -> std::io::Result<()> {
    println!("Starting the web server...");
    HttpServer::new(|| {
        App::new()
            .service(api_endpoints::get_last_massage)
            .service(api_endpoints::write_message)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
