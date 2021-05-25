use std::os::unix::fs::symlink;
use std::{env, fs};
use tera::{Context, Tera};

use wmrio::{content, path_manager::PathManager};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pm = PathManager::from_env()?;

    env::set_current_dir(&pm.project_root)?;

    let template_path = format!("{}/templates/**/*.html", pm.theme_root().display());

    let tera = Tera::new(&template_path)?;
    let site = content::parse_site()?;

    // write out, and then server
    fs::create_dir_all(&pm.out_path())?;
    fs::create_dir_all(format!("{}/posts", &pm.out_path().display()))?;

    for post in &site.posts {
        let rendered_post = tera.render("post.html", &Context::from_serialize(post)?)?;
        let mut post_path = pm.out_path();
        post_path.push("posts");

        let f_out = format!("{}/{}.html", post_path.display(), &post.slug);
        fs::write(f_out, rendered_post)?;
    }

    let out_index = format!("{}/index.html", pm.out_path().display());
    let rendered_index = tera.render("index.html", &Context::from_serialize(&site)?)?;
    fs::write(out_index, rendered_index)?;

    // link static asset dir to build asset
    // Not: this is only sutable for the serve cmd.
    // For building the final asset we will need to copy this dir.
    let out_static = pm.out_static_path();
    if !out_static.exists() {
        symlink(&pm.theme_static_path(), out_static)?;
    }

    println!("listening on 'http://localhost:3030'");

    warp::serve(warp::fs::dir("target/site"))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
