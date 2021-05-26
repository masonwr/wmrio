use std::path::Path;
use std::{env, fs, io};
use tera::{Context, Tera};

use crate::{content::Site, path_manager::PathManager};

pub fn build_site(site: &Site) -> anyhow::Result<()> {
    // set up working dir, simplifies paths a bit
    let pm = PathManager::from_env()?;
    env::set_current_dir(&pm.project_root)?;

    // build tera templater
    // TODO: derive the template path from the incoming site struct
    // parsed from config
    let template_path = format!("{}/templates/**/*.html", pm.theme_root().display());
    let tera = Tera::new(&template_path)?;

    // create dirs if they do not exist
    // TODO this should also be derived from the site perhaps
    let posts_dir = format!("{}/posts", &pm.out_path().display());
    fs::create_dir_all(&posts_dir)?;

    // render posts
    for post in &site.posts {
        let mut context = Context::from_serialize(post)?;
        context.insert("site_title", &site.site_title);

        let rendered_post = tera.render("post.html", &context)?;

        let f_out = format!("{}/{}.html", &posts_dir, &post.slug);
        fs::write(f_out, rendered_post)?;
    }

    // render index.html
    let out_index = format!("{}/index.html", pm.out_path().display());
    let rendered_index = tera.render("index.html", &Context::from_serialize(&site)?)?;
    fs::write(out_index, rendered_index)?;

    // cp static asset folder
    copy_dir_all(&pm.theme_static_path(), pm.out_static_path())?;

    Ok(())
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
