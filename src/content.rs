use serde::Serialize;

pub fn parse_site() -> anyhow::Result<Site> {
    let posts: Vec<Post> = vec![
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
    ];

    let site = Site {
        title: "wmrio".into(),
        posts,
    };

    Ok(site)
}

#[derive(Serialize)]
pub struct Site {
    pub title: String,
    pub posts: Vec<Post>,
}

#[derive(Serialize)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub content: String,
}
