extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate askama;
extern crate env_logger;

mod handlers;

use actix_web::{http::Method, middleware, server::HttpServer, App};
use handlers::*;
use std::env;

fn main() {
    let sys = actix::System::new("deciduously-com");
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .route("/", Method::GET, index)
    }).bind("127.0.0.1:3000")
        .expect("Cannot bind to port 3000")
        .start();

    let _ = sys.run();
}
