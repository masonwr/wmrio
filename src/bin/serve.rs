use std::os::unix::fs::symlink;
use std::{env, fs};
use tera::{Context, Tera};

use wmrio::path_manager::PathManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pm = PathManager::from_env()?;

    env::set_current_dir(&pm.project_root)?;

    let template_path = format!("{}/templates/**/*.html", pm.theme_root().display());

    let tera = Tera::new(&template_path)?;

    let mut context = Context::new();
    context.insert("title", "wmrio");
    context.insert("world", "World");

    let rendered = tera.render("index.html", &context)?;

    // write out, and then server
    fs::create_dir_all(&pm.out_path())?;
    let out_index = format!("{}/index.html", pm.out_path().display());
    fs::write(out_index, rendered)?;

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
