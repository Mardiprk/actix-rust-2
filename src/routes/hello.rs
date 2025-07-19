use actix_web::{get, HttpResponse, Result};

#[get("/")]
async fn hello() -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("YOLO from Rusty Dev!"))
}