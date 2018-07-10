use actix_web::{Error, HttpRequest, HttpResponse, Path, Responder};

#[derive(Serialize)]
struct IndexContext {}

#[derive(Serialize)]
struct PostContext {}

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
    let path = format!("posts/{}.html", post.into_inner());
    println!("{}", path);
    let body = super::TERA
        .render(&path, &PostContext {})
        .expect("Could not render post");
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
