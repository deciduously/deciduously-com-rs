use actix_web::{self, AsyncResponder, HttpRequest, HttpResponse, Path};
use futures::{future::result, Future};
use publish::{base_file_name, file_names};

#[derive(Serialize)]
struct IndexContext {}

impl IndexContext {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize)]
struct PostContext {}

impl PostContext {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize)]
struct PostLink {
    name: String,
}

impl PostLink {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Serialize)]
struct PostsContext {
    posts: Vec<PostLink>,
}

impl PostsContext {
    fn new(posts: Vec<String>) -> Self {
        let mut ret = Vec::new();
        for p in posts {
            ret.push(PostLink::new(p));
        }
        Self { posts: ret }
    }
}

pub fn index(_req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let body = super::TERA
        .render("index.html", &IndexContext::new())
        .unwrap();

    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}

// Eventually, have a /drafts endpoint that can show the draft md files

fn get_post_links() -> Vec<String> {
    let posts_dir = "templates/posts";
    let names: Vec<String> = file_names(posts_dir)
        .unwrap_or(vec!["nada.html".into()])
        .iter()
        .map(|f| (*base_file_name(&*f).unwrap()).into())
        .collect();
    names
}

// GET /posts
pub fn get_posts(_req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let body = super::TERA
        .render("posts.html", &PostsContext::new(get_post_links()))
        .unwrap();
    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}

// GET /post/<title>
pub fn get_post(post: Path<String>) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    // TODO return a 404 if not found
    let path = format!("posts/{}.html", post.into_inner());
    let body = super::TERA.render(&path, &PostContext::new()).unwrap();
    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}
