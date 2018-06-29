use actix_web::Error;
use pulldown_cmark::{html, Parser};
use std::{
    fs::File, io::{prelude::*, BufReader}, path::PathBuf,
};

// Take a post name, look for it in posts/, and returns the parsed HTML string
pub fn bake(f: &str) -> Result<String, Error> {
    // Open the file and read it to markdown_str
    let path = PathBuf::from(&format!("posts/{}.md", f));
    let hello = File::open(path)?;
    let mut reader = BufReader::new(hello);
    let mut markdown_str = String::new();
    // read_to_string returns the number of bytes read - we aren't using that.
    let _ = reader.read_to_string(&mut markdown_str);

    // Parse the markdown to HTML
    let parser = Parser::new(&markdown_str);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    Ok(html_buf)
}
