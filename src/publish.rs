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

pub fn publish() -> io::Result<()> {
    // pop a brand new child template in posts/
    // for every .md in drafts that doesnt have a match
    // To re-publish, we'll need to specify the specific post name
    // For starters, though, just get this working

    println!("Publishing drafts...");

    // make /posts/ if it doesn't exist
    // check if it is
    if !Path::new("./posts").exists() {
        // TODO use a real logger
        println!("No posts directory present - creating...");
        fs::create_dir("./posts")?;
    }

    let drafts = file_names("./drafts/")?;
    let posts = file_names("./posts/")?;

    // Find the drafts with no matching post

    println!("Drafts: {:?}\nPosts: {:?}", drafts, posts);
    Ok(())
}
