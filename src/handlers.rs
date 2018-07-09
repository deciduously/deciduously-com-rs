use actix_web::{Error, HttpRequest, HttpResponse, Path, Responder};
use askama::Template;
use markdown::bake;

// this couldn't be a &'a str because of the impl Responder
#[derive(Template)]
#[template(path = "skel.html")]
struct PageWrapperTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

// TODO This boilerplate feels automatable
impl Responder for IndexTemplate {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = self.render().unwrap();

        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    }
}

pub fn index(_req: HttpRequest) -> impl Responder {
    IndexTemplate {}
}

// Eventually, have a /drafts endpoint that can show the draft md files

// GET /post/<title>
pub fn get_post(post: Path<String>) -> Result<HttpResponse, Error> {
    // TODO return a 404 if not found
    // TODO how do we compile a template on the fly
    let html_post = "<h1>broken</h1>";
    Ok(HttpResponse::Ok().content_type("text/html").body(html_post))
}
