use markdown::bake;
use std::{fs, io, path::Path};

fn file_names(path: &str) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|file| {
            file.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
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

pub fn publish() -> io::Result<()> {
    // pop a brand new child template in posts/
    // for every .md in drafts that doesnt have a match
    // To re-publish, we'll need to specify the specific post name
    // For starters, though, just get this working

    println!("Publishing drafts...");

    // TEMPORARILY delete posts first - we'll just rebuild everything
    println!("!!Purging posts directory!!");
    fs::remove_dir_all("./posts")?;

    // make /posts/ if it doesn't exist
    // check if it is
    if !Path::new("./posts").exists() {
        // TODO use a real logger
        println!("No posts directory present - creating...");
        fs::create_dir("./posts")?;
    }

    let drafts = file_names("./drafts/")?;
    //let posts = file_names("./posts/")?;

    // Find the drafts with no matching post

    // FOR NOW lets just do all of them
    for draft in drafts {
        // we want to markdown it.

        let output_name = switch_ext_md_to_html(&draft)?;
        println!("draft: {}\n{}", &draft, &output_name);
        let rendered = bake(base_file_name(&draft)?).expect("Could not render selected draft");
        // save it to posts
        let _ = fs::write(format!("./posts/{}", &output_name), rendered);
    }

    Ok(())
}
