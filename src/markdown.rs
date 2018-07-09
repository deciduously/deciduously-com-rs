use pulldown_cmark::{html, Parser};
use std::{
    fs::File, io::{prelude::*, BufReader, Result}, path::PathBuf,
};

// Take a post name, look for it in posts/, and return the parsed HTML string
pub fn bake(f: &str) -> Result<String> {
    // Open the file
    let path = PathBuf::from(&format!("drafts/{}.md", f));
    let md_file = File::open(path)?;

    // Read the contents to a string
    let mut reader = BufReader::new(md_file);
    let mut md_string = String::new();
    // read_to_string returns the number of bytes read - we aren't using that.
    let _ = reader.read_to_string(&mut md_string);

    // Parse the markdown to HTML
    let parser = Parser::new(&md_string);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);

    // Send it on up
    Ok(html_buf)
}
