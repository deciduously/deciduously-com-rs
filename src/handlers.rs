use actix_web::{HttpRequest, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]

// this can be named anything
struct HelloTemplate<'a> {
    name: &'a str,
}

pub fn index(_req: HttpRequest) -> Result<String> {
    let hello = HelloTemplate { name: "world" };
    Ok(format!("{}", hello.render().unwrap()))
}
