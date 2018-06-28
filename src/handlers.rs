use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]

// this can be named anything
struct IndexTemplate<'a> {
    name: &'a str,
}

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
