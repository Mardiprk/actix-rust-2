use actix_web::{HttpServer, Responder, get, App, web, post, HttpResponse};
use serde::{Deserialize};

#[derive(Deserialize)]
struct GreetRequest{
    name: String
}

#[get("/")]
async fn home() -> impl Responder{
    "Homepage"
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder{
    format!("Hello, {name}")
}

#[post("/greet")]
async fn greet(req: web::Json<GreetRequest>) -> impl Responder {
    let name = &req.name;
    HttpResponse::Ok().body(format!("Hello, {name}!"))
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Starting Server http://127.0.0.1:8080");

    HttpServer::new(||{
        App::new()
        .service(home)
        .service(greet)
        .service(hello)
    }).bind(("127.0.0.1",8080))?
    .run()
    .await
}