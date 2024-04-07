use std::fs;
use actix_web::{get, HttpResponse};
#[cfg(feature = "inline")]
use crate::inline;

#[get("/")]
async fn home() -> HttpResponse {
    #[cfg(not(feature = "inline"))]
    return HttpResponse::Ok().body(fs::read_to_string("web/home.html").unwrap());
    #[cfg(feature = "inline")]
    return HttpResponse::Ok().body(inline::get_home());
}
