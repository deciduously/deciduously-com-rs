use markdown::bake;
use std::{fs, io, path::Path};

fn file_names(path: &str) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|file| {
            file.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(String::from))
            })
        })
        .collect::<Vec<String>>())
}

// Again, PathBuf
fn base_file_name(p: &str) -> io::Result<&str> {
    let parts: Vec<&str> = p.split('.').collect();
    Ok(parts[0])
}

// TODO this is a little hacky - prefer PathBuf
fn switch_ext_md_to_html(p: &str) -> io::Result<String> {
    Ok(format!("{}.html", base_file_name(p)?))
}

// Takes an HTML string and surrounds it with template boilerplate
fn wrap_content(content: String) -> String {
    let prefix = "{% block content %}";
    let postfix = "{% endblock %}";
    // You may need to kill the second \n
    format!("{}\n{}\n{}", prefix, content, postfix)
}

pub fn publish() -> io::Result<()> {
    println!("Publishing drafts...");

    // TEMPORARILY delete posts first - we'll just rebuild everything
    println!("!!Purging posts directory!!");
    fs::remove_dir_all("./posts")?;

    // make /posts/ if it doesn't exist
    if !Path::new("./posts").exists() {
        // TODO use a real logger
        println!("No posts directory present - creating...");
        fs::create_dir("./posts")?;
    }

    let drafts = file_names("./drafts/")?;
    for draft in drafts {
        let output_name = switch_ext_md_to_html(&draft)?;
        println!("draft: {}\n{}", &draft, &output_name);
        let rendered =
            wrap_content(bake(base_file_name(&draft)?).expect("Could not render selected draft"));

        // save it to posts
        let _ = fs::write(format!("./posts/{}", &output_name), rendered);
    }

    Ok(())
}
