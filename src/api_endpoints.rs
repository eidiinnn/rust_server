use crate::db::get_connection;
use crate::db_messages;
use actix_web::{
    get,
    post,
    web::{self, block},
    HttpResponse,
};
use serde::Deserialize;

#[get("/api/getLastMessage")]
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
        Ok(json) => HttpResponse::Ok().append_header(("content-type", "json")).body(json.unwrap()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct WriteMessageData {
    message: String,
}
#[post("/api/writeMessage")]
async fn write_message(req: web::Json<WriteMessageData>) -> HttpResponse {
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
