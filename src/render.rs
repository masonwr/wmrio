use serde::Serialize;
use std::path::Path;
use std::{env, fs, io};
use tera::{Context, Tera};

use crate::{content::Site, path_manager::PathManager};

pub fn site(site: &Site) -> anyhow::Result<()> {
    // set up working dir, simplifies paths a bit
    let pm = PathManager::from_env()?;
    env::set_current_dir(&pm.project_root)?;

    // build tera templater
    // TODO: derive the template path from the incoming site struct
    // parsed from config
    let template_path = format!("{}/templates/**/*.html", pm.theme_root().display());
    let mut tera = Tera::new(&template_path)?;
    tera.autoescape_on(vec![]);

    // create dirs if they do not exist
    // TODO this should also be derived from the site perhaps
    let posts_dir = format!("{}/posts", &pm.out_path().display());
    fs::create_dir_all(&posts_dir)?;

    let mut base_context = Context::from_serialize(&site)?;
    base_context.extend(Context::from_serialize(parse_theme_config())?);

    // render posts
    for post in &site.posts {
        // this works as long as the posts all have the same fields
        // because we are overwriting the keys
        base_context.extend(Context::from_serialize(post)?);

        let rendered_post = tera.render("post.html", &base_context)?;

        let f_out = format!("{}/{}.html", &posts_dir, &post.meta.slug);
        fs::write(f_out, rendered_post)?;
    }

    // render index.html
    let out_index = format!("{}/index.html", pm.out_path().display());
    let rendered_index = tera.render("index.html", &base_context)?;
    fs::write(out_index, rendered_index)?;

    // cp static asset folder
    copy_dir_all(&pm.theme_static_path(), pm.out_static_path())?;

    Ok(())
}

#[derive(Serialize)]
pub struct ThemeConfig {
    pub top_nav: Vec<NavItem>,
}
#[derive(Serialize)]
pub struct NavItem {
    pub display: String,
    pub link: String,
}

fn parse_theme_config() -> ThemeConfig {
    ThemeConfig {
        top_nav: vec![
            NavItem {
                display: "Github".into(),
                link: "https://github.com/masonwr".into(),
            },
            NavItem {
                display: "About".into(),
                link: "/posts/about.html".into(),
            },
        ],
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
