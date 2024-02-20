mod db;
use db::DB;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};


fn main() {
    let mut database = DB::new();
    database.create_message_table();

    let _ = server();
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn server()-> std::io::Result<()> {
   HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
