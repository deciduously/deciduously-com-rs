use actix_web::{self, AsyncResponder, HttpRequest, HttpResponse, Path};
use errors::{self, *};
use futures::{future::result, Future};
use publish::{base_file_name, file_names};
use std::{fmt, str::FromStr};

#[derive(Serialize)]
struct DemosContext {
    demos: Vec<DemoLink>,
}

impl DemosContext {
    fn new(demos: Vec<(String, String)>) -> Self {
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
    name: String,
}

impl DemoLink {
    fn new((name, description): (String, String)) -> Self {
        Self { description, name }
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
struct NotFoundContext {
    name: String,
}

impl NotFoundContext {
    fn new(name: String) -> Self {
        Self { name }
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

fn get_demo_links() -> Vec<(String, String)> {
    vec![
        Extern::Dots.get_link_text(),
        Extern::Impact.get_link_text(),
        Extern::Mines.get_link_text(),
    ]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Extern {
    Dots,
    Impact,
    Mines,
}

impl Extern {
    fn get_description(self) -> String {
        match self {
            Extern::Dots => "A WASM clone of the flash game Boomshine".into(),
            Extern::Impact => "A WASM incremental game skeleton implemented in Yew".into(),
            Extern::Mines => "A ClojureScript clone of Minesweeper - broken!".into(),
        }
    }

    fn get_link_text(self) -> (String, String) {
        (format!("{}", self), self.get_description())
    }
}

impl fmt::Display for Extern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Extern::*;
        let s = match *self {
            Dots => "dots",
            Impact => "impact",
            Mines => "mines",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Extern {
    type Err = Error;

    fn from_str(s: &str) -> errors::Result<Self> {
        match s {
            "dots" | "Dots" => Ok(Extern::Dots),
            "impact" | "Impact" => Ok(Extern::Impact),
            "mines" | "Mines" => Ok(Extern::Mines),
            _ => bail!("Not a known extern"),
        }
    }
}

pub fn not_found(path: Path<String>) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    result(Ok(HttpResponse::NotFound().content_type("text/html").body(
        super::TERA
            .render("404.html", &NotFoundContext::new(path.into_inner()))
            .unwrap(),
    ))).responder()
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
    let path = page.clone();
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
        _ => return not_found(page),
    };
    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}

// GET /
pub fn index(_req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let body = super::TERA
        .render("index.html", &EmptyContext::new())
        .unwrap();

    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}
