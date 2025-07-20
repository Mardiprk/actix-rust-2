use actix_web::{get, HttpResponse, Result};

#[get("/welcome")]
async fn welcome() -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body("Welcome to Actix Dev!"))
}