use actix_web::{Error, HttpRequest, HttpResponse, Path, Responder};

#[derive(Serialize)]
struct IndexContext {}

impl Responder for IndexContext {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = super::TERA
            .render("index.html", &self)
            .expect("Problem rendering index");

        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    }
}

pub fn index(_req: HttpRequest) -> impl Responder {
    IndexContext {}
}

// Eventually, have a /drafts endpoint that can show the draft md files

// GET /post/<title>
pub fn get_post(post: Path<String>) -> Result<HttpResponse, Error> {
    // TODO return a 404 if not found
    // TODO how do we compile a template on the fly
    let html_post = "<h1>broken</h1>";
    Ok(HttpResponse::Ok().content_type("text/html").body(html_post))
}
