use actix_web::{Error, HttpRequest, HttpResponse, Path, Responder};
use askama::Template;
use markdown::bake;

// this couldn't be a &'a str because of the impl Responder
#[derive(Template)]
#[template(path = "index.html")]
struct PageWrapperTemplate {
    page: String,
}

// TODO This boilerplate feels automatable
impl Responder for PageWrapperTemplate {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = self.render().unwrap();

        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    }
}

pub fn index(_req: HttpRequest) -> impl Responder {
    PageWrapperTemplate {
        page: "Index Page".to_string(),
    }
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

// TODO here's part of your problem
pub fn parse_md(post: Path<String>) -> impl Responder {
    // TODO return a 404 if not found
    PageWrapperTemplate {
        page: bake(&post).expect("Could not bake requested markdown resource"),
    }
}
