pub mod hello;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig){
    cfg.service(hello::hello);
}