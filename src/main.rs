extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate askama;
extern crate env_logger;
extern crate pulldown_cmark;

mod handlers;
mod markdown;

use actix_web::{http::Method, middleware, server::HttpServer, App};
use handlers::*;
use std::{env, process};

enum Cmd {
    Usage,
    Publish,
    Serve,
}

impl Cmd {
    fn run(&self) {
        match self {
            Cmd::Usage => usage(),
            Cmd::Publish => publish(),
            Cmd::Serve => serve(),
        }
    }
}

fn publish() {
    unimplemented!()
}

fn serve() {
    let sys = actix::System::new("deciduously-com");
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .route("/", Method::GET, index)
            .resource("/post/{post_name}", |r| {
                r.method(Method::GET).with(parse_md)
            })
    }).bind("127.0.0.1:8080")
        .expect("Cannot bind to port 8080")
        .start();

    let _ = sys.run();
}

fn usage() {
    // TODO add verison
    println!("deciduously-com\nSupported operations: help | publish | serve\ne.g.: deciduously-com publish or cargo run -- publish");
    process::exit(0);
}

fn main() {
    let cmd = if let Some(arg) = env::args().nth(1) {
        match arg.as_ref() {
            "publish" => Cmd::Publish,
            "serve" => Cmd::Serve,
            "help" => Cmd::Usage,
            _ => {
                eprintln!(
                    "Unrecognized operation: {}\nSupported operations: help | publish | serve",
                    arg
                );
                process::exit(1);
            }
        }
    } else {
        Cmd::Serve
    };

    cmd.run();
}
