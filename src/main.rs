extern crate actix;
extern crate actix_web;
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
mod publish;

use actix_web::{
    fs::StaticFiles,
    http,
    middleware::{self, cors::Cors},
    server::HttpServer,
    App,
};
use errors::*;
use handlers::{get_demo, get_post, get_template, index};
use publish::publish;
use std::{env, process, str::FromStr};
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

enum Build {
    Dev,
    Prod,
}

impl FromStr for Build {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "dev" | "DEV" => Build::Dev,
            "prod" | "PROD" => Build::Prod,
            _ => Build::Dev,
        })
    }
}

fn get_build_config() -> Result<Build> {
    Build::from_str(&::std::env::var("BUILD").unwrap_or_else(|_| "NOTSET".into()))
}

fn serve() -> Result<()> {
    // get_vars DEV 127.0.0.1:8080, PROD
    let addr = match get_build_config()? {
        Build::Dev => "127.0.0.1:8080",
        Build::Prod => "0.0.0.0:80",
    };

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
                        .resource("/{page}", |r| r.route().with(get_template))
                        .resource("/demo/{demo}", |r| {
                            r.method(http::Method::GET).with(get_demo)
                        })
                        .resource("/post/{post}", |r| {
                            r.method(http::Method::GET).with(get_post)
                        })
                        .register()
                }
            })
            .handler("/static", StaticFiles::new("./static/").unwrap())
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
