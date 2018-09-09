extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate pulldown_cmark;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate tera;

mod errors {
    error_chain!{}
}
mod handlers;
mod markdown;
mod publish;

use actix_web::{
    http,
    middleware::{self, cors::Cors},
    server::HttpServer,
    App,
};
use errors::*;
use handlers::{get_post, get_posts, index};
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
    fn run(&self) -> Result<()> {
        match self {
            Cmd::Usage => {
                usage()?;
            }
            Cmd::Publish => {
                publish()?;
            }
            Cmd::Serve => {
                serve()?;
            }
        }
        Ok(())
    }
}

fn serve() -> Result<()> {
    let addr = "127.0.0.1:8080";

    let sys = actix::System::new("deciduously-com");

    HttpServer::new(move || {
        App::new()
            .configure({
                |app| {
                    Cors::for_app(app)
                        .send_wildcard()
                        .allowed_methods(vec!["GET"])
                        .max_age(3600)
                        .resource("/", |r| r.route().a(index))
                        .resource("/posts", |r| r.route().a(get_posts))
                        .resource("/post/{post}", |r| {
                            r.method(http::Method::GET).with(get_post)
                        })
                        .register()
                }
            })
            .middleware(middleware::Logger::default())
    }).bind(addr)
        .chain_err(|| "Could not initialize server")?
        .start();

    let _ = sys.run();
    Ok(())
}

fn usage() -> Result<()> {
    println!("deciduously-com v0.2.0\nSupported operations: help | publish | serve\ne.g.: deciduously-com publish or cargo run -- publish");
    process::exit(0);
}

fn run() -> Result<()> {
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

    cmd.run()?;
    Ok(())
}

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    // Immediately call into a properly error-chained fn
    if let Err(ref e) = run() {
        error!("error: {}", e);

        for e in e.iter().skip(1) {
            debug!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            trace!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
