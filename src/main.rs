use actix_web::{HttpServer, Responder, get, App};

#[get("/")]
async fn hello() -> impl Responder{
    "Hello, Actix Web!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Starting Server at http://127.0.0.1:8080");

    HttpServer::new(||{
        App::new().service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}