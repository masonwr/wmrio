use std::os::unix::fs::symlink;
use std::{env, fs};
use tera::{Context, Tera};

use crate::{content::Site, path_manager::PathManager};

pub fn build_site(site: &Site) -> anyhow::Result<()> {
    // set up working dir, simplifies paths a bit
    let pm = PathManager::from_env()?;
    env::set_current_dir(&pm.project_root)?;

    // build tera templater
    // todo: derive the template path from the incoming site struct
    // parsed from config
    let template_path = format!("{}/templates/**/*.html", pm.theme_root().display());
    let tera = Tera::new(&template_path)?;

    // create dirs if they do not exist
    // TODO this should also be derived from the site perhaps
    let posts_dir = format!("{}/posts", &pm.out_path().display());
    fs::create_dir_all(&posts_dir)?;

    // render posts
    for post in &site.posts {
        let rendered_post = tera.render("post.html", &Context::from_serialize(post)?)?;
        let f_out = format!("{}/{}.html", &posts_dir, &post.slug);
        fs::write(f_out, rendered_post)?;
    }

    // render index.html
    let out_index = format!("{}/index.html", pm.out_path().display());
    let rendered_index = tera.render("index.html", &Context::from_serialize(&site)?)?;
    fs::write(out_index, rendered_index)?;

    // link static asset dir to build asset
    // TODO update this to copy assets
    let out_static = pm.out_static_path();
    if !out_static.exists() {
        symlink(&pm.theme_static_path(), out_static)?;
    }

    Ok(())
}
