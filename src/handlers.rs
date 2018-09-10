use actix_web::{self, AsyncResponder, HttpRequest, HttpResponse, Path};
use futures::{future::result, Future};
use publish::{base_file_name, file_names};

#[derive(Serialize)]
struct DemosContext {
    demos: Vec<DemoLink>,
}

impl DemosContext {
    fn new(demos: Vec<(String, String, String)>) -> Self {
        let mut ret = Vec::new();
        for d in demos {
            ret.push(DemoLink::new(d));
        }
        Self { demos: ret }
    }
}

#[derive(Serialize)]
struct DemoLink {
    description: String,
    link: String,
    name: String,
}

impl DemoLink {
    fn new((name, link, description): (String, String, String)) -> Self {
        Self {
            description,
            link,
            name,
        }
    }
}

#[derive(Serialize)]
struct EmptyContext {}

impl EmptyContext {
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
        .render("index.html", &EmptyContext::new())
        .unwrap();

    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}

fn get_demo_links() -> Vec<(String, String, String)> {
    vec![(
        "dots".into(),
        "../static/extern/dots/index.html".into(),
        "A WASM clone of the flash game Boomshine".into(),
    )]
}

// Eventually, have a /drafts endpoint that can show the draft md files

fn get_post_links() -> Vec<String> {
    let posts_dir = "templates/posts";
    let names: Vec<String> = file_names(posts_dir)
        .unwrap_or_else(|_| vec!["nada.html".into()])
        .iter()
        .map(|f| (*base_file_name(&*f).unwrap()).into())
        .collect();
    names
}

// GET /demos/<demo>
pub fn get_demo(demo: Path<String>) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let path = demo.into_inner();
    let body = match path.as_str() {
        "dots" => super::TERA
            .render("extern/dots/index.html", &EmptyContext::new())
            .unwrap(),
        _ => format!("<h3>I haven't written anything called {}!</h3>", path),
    };
    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}

// GET /post/<title>
pub fn get_post(post: Path<String>) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    // TODO return a 404 if not found
    let path = format!("posts/{}.html", post.into_inner());
    let body = super::TERA.render(&path, &EmptyContext::new()).unwrap();
    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}

// GET /{page}
pub fn get_template(
    page: Path<String>,
) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let path = page.into_inner();
    let body = match path.as_str() {
        "contact" => super::TERA
            .render("contact.html", &EmptyContext::new())
            .unwrap(),
        "demos" => super::TERA
            .render("demos.html", &DemosContext::new(get_demo_links()))
            .unwrap(),
        "posts" => super::TERA
            .render("posts.html", &PostsContext::new(get_post_links()))
            .unwrap(),
        _ => super::TERA
            .render("404.html", &EmptyContext::new())
            .unwrap(),
    };
    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}
