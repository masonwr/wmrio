use serde::Serialize;

pub fn parse_site() -> anyhow::Result<Site> {
    let site = Site {
        site_title: "wmr.io".into(),
        posts: vec![
            Post {
                slug: "post1".into(),
                title: "this is a test".into(),
                content: "here is the content".into(),
            },
            Post {
                slug: "post2".into(),
                title: "another test here".into(),
                content: "what what".into(),
            },
        ],
    };

    Ok(site)
}

#[derive(Serialize)]
pub struct Site {
    pub site_title: String,
    pub posts: Vec<Post>,
}

#[derive(Serialize)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub content: String,
}
