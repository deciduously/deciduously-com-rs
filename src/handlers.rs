use actix_web::{Error, HttpRequest, HttpResponse, Path, Responder};
use askama::Template;
use markdown::bake;

#[derive(Template)]
#[template(path = "index.html")]

// this can be named anything
struct IndexTemplate<'a> {
    name: &'a str,
}

// TODO This boilerplate feels automatable
impl<'a> Responder for IndexTemplate<'a> {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = self.render().unwrap();

        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    }
}

pub fn index(_req: HttpRequest) -> impl Responder {
    IndexTemplate { name: "world" }
}

// BEN - you should be baking these ahead of time for production serving.
// No reason to run the parser live - it wont be changing.

pub fn parse_md(post: Path<String>) -> Result<HttpResponse, Error> {
    // TODO return a 404 if not found
    let html = bake(&post)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
