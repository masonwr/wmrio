use crate::path_manager::PathManager;
use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};
use std::fs;

const FRONT_MATTER_DELIMETER: &str = "---";

pub fn parse_site() -> anyhow::Result<Site> {
    let pm = PathManager::from_env()?;

    // parse posts
    let posts_dir = format!("{}/content/posts", &pm.project_root);
    let posts: Vec<Content> = parse_content(&posts_dir)?;

    let pages_dir = format!("{}/content/pages", &pm.project_root);
    let pages = parse_content(&pages_dir)?;

    let site = Site {
        site_title: "wmrio".into(),
        posts,
        pages,
    };

    Ok(site)
}

fn parse_content(posts_dir: &str) -> anyhow::Result<Vec<Content>> {
    let mut parsing_opts = ComrakOptions::default();
    parsing_opts.extension.front_matter_delimiter = Some(FRONT_MATTER_DELIMETER.to_owned());

    let mut posts: Vec<Content> = vec![];

    for file in fs::read_dir(posts_dir)? {
        let file = file?;
        let ftype = file.file_type()?;
        if ftype.is_dir() {
            continue;
        }

        // parse file into post
        let content = fs::read_to_string(file.path())?;
        let meta = parse_content_meta(&content)
            .map_err(|e| anyhow::anyhow!("{}: {}", &file.path().display(), e))?;

        // parse markdown
        let content = markdown_to_html(&content, &parsing_opts);
        posts.push(Content { meta, content });
    }

    Ok(posts)
}

fn parse_content_meta(content: &str) -> anyhow::Result<ContentMeta> {
    let front_matter =
        get_yaml(&content).ok_or_else(|| anyhow::anyhow!("missing or malformed formed front"))?;

    let meta: ContentMeta = serde_yaml::from_str(&front_matter)
        .map_err(|e| anyhow::anyhow!("malformed front matter: {}", e,))?;

    Ok(meta)
}

// get_yaml returns a slice from the post targeting the front matter yaml
fn get_yaml<'a>(text: &'a str) -> Option<&'a str> {
    let delimeter = format!("{}\n", FRONT_MATTER_DELIMETER);
    let text = text.trim_start();

    match text.starts_with(&delimeter) {
        true => {
            let after_start_delimeter = &text[4..];
            match after_start_delimeter.find(&delimeter) {
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
    pub posts: Vec<Content>,
    pub pages: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentMeta {
    pub title: String,
    pub slug: String,
    pub status: Option<Status>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Published,
}

#[derive(Serialize)]
pub struct Content {
    pub meta: ContentMeta,
    pub content: String,
}
