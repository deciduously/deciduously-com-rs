extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
extern crate pulldown_cmark;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate tera;

mod handlers;
mod markdown;
mod publish;

use actix_web::{http::Method, middleware, server::HttpServer, App};
use handlers::*;
use publish::publish;
use std::{env, process};
use tera::Tera;

lazy_static! {
    pub static ref TERA: Tera = compile_templates!("templates/**/*");
}

enum Cmd {
    Usage,
    Publish,
    Serve,
}

impl Cmd {
    fn run(&self) {
        match self {
            Cmd::Usage => usage(),
            Cmd::Publish => {
                publish().unwrap();
            }
            Cmd::Serve => serve(),
        }
    }
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
                r.method(Method::GET).with(get_post)
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
