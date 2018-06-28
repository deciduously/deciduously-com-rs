use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use askama::Template;
use pulldown_cmark::{html, Parser};

#[derive(Template)]
#[template(path = "index.html")]

// this can be named anything
struct IndexTemplate<'a> {
    name: &'a str,
}

// TODO This boilerplate feels automatable
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

// luckily for testing, Strings are Responders
pub fn parse_md(_req: HttpRequest) -> Result<HttpResponse, Error> {
    let markdown_str = r#"
Howdy
# Head 1
## Head 2
* list 1
* **bold** list 2

`inline code` is *one* option
```rust
code blocks are another
```
"#;
    let parser = Parser::new(markdown_str);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    Ok(HttpResponse::Ok().content_type("text/html").body(html_buf))
}
