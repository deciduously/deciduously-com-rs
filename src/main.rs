extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate askama;
extern crate env_logger;
extern crate pulldown_cmark;

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
            .route("/md", Method::GET, parse_md)
    }).bind("127.0.0.1:8080")
        .expect("Cannot bind to port 8080")
        .start();

    let _ = sys.run();
}
