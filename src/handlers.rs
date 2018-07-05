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

// BEN - you should be baking these ahead of time for production serving.
// No reason to run the parser live - it wont be changing.

// Try having a /draft endpoint which runs the parser live like this
// and doenst have a link from the front page
// as well as a /post endpoint which only serves pre-baked posts
// and include a "publish" command in the executable to bake posts to that alternate location,
// removing them from drafts (maybe?)
// either all posts in drafts or a specific post

// ALSO it's not enough to just run the parser
// you need to embed the generated HTML in our page skeleton
// in draft/ it'll happen on the fly
// in post/ it'll be part of the bake process.

pub fn parse_md(post: Path<String>) -> Result<HttpResponse, Error> {
    // TODO return a 404 if not found
    let html_post = bake(&post).expect("Could not bake requested markdown resource");
    Ok(HttpResponse::Ok().content_type("text/html").body(html_post))
}
