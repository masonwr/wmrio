use crate::path_manager::PathManager;
use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};
use std::fs;

pub fn parse_site() -> anyhow::Result<Site> {
    let pm = PathManager::from_env()?;

    let mut parsing_opts = ComrakOptions::default();
    parsing_opts.extension.front_matter_delimiter = Some("---".to_owned());

    // parse posts
    let mut posts: Vec<Post> = vec![];
    let posts_dir = format!("{}/content/posts", &pm.project_root);
    for file in fs::read_dir(posts_dir)? {
        let file = file?;
        let ftype = file.file_type()?;
        if ftype.is_dir() {
            continue;
        }

        // parse file into post
        let content = fs::read_to_string(file.path())?;

        let yaml = get_yaml(&content).ok_or_else(|| {
            anyhow::anyhow!(
                "post '{}' did not have required front matter.",
                file.path().display()
            )
        })?;

        let meta: ContentMeta = serde_yaml::from_str(&yaml).map_err(|e| {
            anyhow::anyhow!(
                "post '{}' had malformed front matter: {}.",
                file.path().display(),
                e,
            )
        })?;

        let html = markdown_to_html(&content, &parsing_opts);

        posts.push(Post {
            meta,
            content: html.to_owned(),
        });
    }

    let site = Site {
        site_title: "wmr.io".into(),
        posts,
    };

    Ok(site)
}

fn get_yaml(text: &str) -> Option<&str> {
    let text = text.trim_start();
    match text.starts_with("---\n") {
        true => {
            let after_start_delimeter = &text[4..];
            match after_start_delimeter.find("---\n") {
                Some(i) => Some(&after_start_delimeter[..i]),
                None => None,
            }
        }
        false => None,
    }
}

#[derive(Serialize)]
pub struct Site {
    pub site_title: String,
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentMeta {
    pub title: String,
    pub slug: String,
}

#[derive(Serialize)]
pub struct Post {
    pub meta: ContentMeta,
    pub content: String,
}
