use actix_web::{get, web, HttpResponse, Result};

#[get("/user/{name}")]
async fn user_data(name: web::Path<String>) -> Result<HttpResponse>{
    Ok(HttpResponse::Ok().body(format!("Hello, {} you're user 1", name)))
}