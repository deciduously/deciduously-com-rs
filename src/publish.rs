// publish.rs turns our markdown drafts into templates/posts
use errors::*;
use markdown::bake;
use std::{fs, path::Path};

pub fn file_names(path: &str) -> Result<Vec<String>> {
    Ok(fs::read_dir(path)
        .chain_err(|| "Could not read drafts folder")?
        .filter_map(|file| {
            file.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(String::from))
            })
        })
        .collect::<Vec<String>>())
}

pub fn base_file_name(p: &str) -> Result<&str> {
    let parts: Vec<&str> = p.split('.').collect();
    Ok(parts[0])
}

fn switch_ext_md_to_html(p: &str) -> Result<String> {
    Ok(format!("{}.html", base_file_name(p)?))
}

// Takes an HTML string and surrounds it with template boilerplate
fn wrap_content(content: &str, title: &str) -> String {
    let prefix = "{% extends \"skel.html\" %}\n\n{% block content %}";
    let title_block = format!("{{% block title %}}{}{{% endblock %}}", title);
    let postfix = "{% endblock %}";
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
