use actix_files::Files;
use actix_web::{App, HttpServer};
use rust_server::{api_endpoints, db, db_messages};

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
        .service(Files::new("/", "./static").index_file("index.html").show_files_listing())
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
