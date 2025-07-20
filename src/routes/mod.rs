pub mod hello;
pub mod welcome;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig){
    cfg
        .service(welcome::welcome)
        .service(hello::hello);
}