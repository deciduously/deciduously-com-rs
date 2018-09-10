// publish.rs turns our markdown drafts into templates/posts
use errors::*;
use pulldown_cmark::{html, Parser};
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

// Take a post name, look for it in drafts/, and return the parsed HTML string
fn bake(f: &str) -> Result<String> {
    // Open the file
    let path = PathBuf::from(&format!("drafts/{}.md", f));
    let md_file =
        File::open(&path).chain_err(|| format!("could not open {}", &path.to_str().unwrap()))?;

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

pub fn file_names(path: &str) -> Result<Vec<String>> {
    Ok(fs::read_dir(path)
        .chain_err(|| "Could not read drafts folder")?
        .filter_map(|file| {
            file.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(String::from))
            })
        }).collect::<Vec<String>>())
}

pub fn base_file_name(p: &str) -> Result<&str> {
    let parts: Vec<&str> = p.split('.').collect();
    Ok(parts[0])
}

fn switch_ext_md_to_html(p: &str) -> Result<String> {
    Ok(format!("{}.html", base_file_name(p)?))
}

// Takes an HTML string and surrounds it with template boilerplate
pub fn wrap_content(content: &str, title: &str) -> String {
    let prism_head =
        "{% block head %}<link href=\"../static/themes/prism.css\" rel=\"stylesheet\" />{% endblock %}";
    let prism_body = "<script src=\"../static/prism.js\"></script>";
    let prefix = format!(
        "{{% extends \"skel.html\" %}}\n\n{}\n\n{{% block content %}}\n\n",
        prism_head
    );
    let title_block = format!("{{% block title %}}<i>{}</i>{{% endblock %}}", title);
    let postfix = format!("{}{{% endblock %}}", prism_body);
    // You may need to kill the second \n
    format!("{}\n{}\n{}\n{}", prefix, title_block, content, postfix)
}

pub fn publish() -> Result<()> {
    info!("Publishing drafts...");

    // delete posts first - we'll just rebuild everything
    // I'd imagine this would have to get huge before this is a major problem
    fs::remove_dir_all("./templates/posts").chain_err(|| "Could not purge posts dir")?;

    // make /posts/
    if !Path::new("./posts").exists() {
        info!("No posts directory present - creating...");
        fs::create_dir("./templates/posts").chain_err(|| "Unable to create posts dir")?;
    }

    let drafts = file_names("./drafts/")?;
    for draft in drafts {
        let output_name = switch_ext_md_to_html(&draft)?;
        debug!("draft: {}\n{}", &draft, &output_name);
        let rendered = wrap_content(
            &bake(base_file_name(&draft)?).chain_err(|| "Markdown parsing problem")?,
            base_file_name(&draft)?,
        );

        // save it to posts
        let _ = fs::write(format!("./templates/posts/{}", &output_name), rendered);
    }

    Ok(())
}
