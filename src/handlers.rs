use actix_web::{self, AsyncResponder, HttpRequest, HttpResponse, Path};
use errors::{self, *};
use futures::{future::result, Future};
use publish::{base_file_name, file_names, wrap_content};
use std::{
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
    str::FromStr,
};

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

pub fn index(_req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let body = super::TERA
        .render("index.html", &EmptyContext::new())
        .unwrap();

    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}

fn get_demo_links() -> Vec<(String, String, String)> {
    vec![Extern::Dots.get_link_text(), Extern::Mines.get_link_text()]
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
enum Extern {
    Dots,
    Mines,
}

impl Extern {
    fn get_link(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(".");
        path.push("static");
        path.push("extern");
        path.push(&format!("{}", self));
        path.push("index.html");
        path
    }

    fn get_link_text(&self) -> (String, String, String) {
        match self {
            Extern::Dots => (
                "dots".into(),
                self.get_link().to_str().unwrap().into(),
                "A WASM clone of the flash game Boomshine".into(),
            ),
            Extern::Mines => (
                "mines".into(),
                self.get_link().to_str().unwrap().into(),
                "A Reagent clone of Minesweeper - currently unfinished and broken!   Whee.".into(),
            ),
        }
    }
}

impl fmt::Display for Extern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Extern::*;
        let s = match *self {
            Dots => "dots",
            Mines => "mines",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Extern {
    type Err = Error;

    fn from_str(s: &str) -> errors::Result<Self> {
        match s {
            "dots" => Ok(Extern::Dots),
            "mines" => Ok(Extern::Mines),
            _ => bail!("Not a known extern"),
        }
    }
}

// given the index hile as a relative path
// get_extern_file returns the HTML to return
// wrapped in a Tera template
fn get_extern_file(e: Extern) -> errors::Result<String> {
    let title = format!("{}", e);

    let path = e.get_link();

    // open path and read to String
    let f = File::open(path).chain_err(|| "could not open extern's index file")?;
    let mut bfr = BufReader::new(f);
    let mut raw = String::new();
    bfr.read_to_string(&mut raw)
        .chain_err(|| "could not read extern's index file")?;
    Ok(wrap_content(&raw, &title))
}

// GET /demos/<demo>
pub fn get_demo(demo: Path<String>) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let path = demo.into_inner();
    let path_str = path.as_str();
    let body = get_extern_file(Extern::from_str(path_str).unwrap()).unwrap_or_else(|_| {
        super::TERA
            .render("404.html", &NotFoundContext::new(path_str.into()))
            .unwrap()
    });
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
            .render("404.html", &NotFoundContext::new(path.as_str().into()))
            .unwrap(),
    };
    result(Ok(HttpResponse::Ok().content_type("text/html").body(body))).responder()
}
