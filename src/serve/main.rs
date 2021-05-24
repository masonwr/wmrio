use std::os::unix::fs::symlink;
use std::path::Path;
use std::{env, fs};
use tera::{Context, Tera};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project_root = env::var("CARGO_MANIFEST_DIR")?;
    env::set_current_dir(&project_root)?;

    let theme_path = format!("{}/themes/default", &project_root);
    let template_path = format!("{}/templates/**/*.html", theme_path);
    let tera = Tera::new(&template_path)?;

    let mut context = Context::new();
    context.insert("title", "wmrio");
    context.insert("world", "World");

    let rendered = tera.render("index.html", &context)?;

    // write out, and then server
    fs::create_dir_all("target/site")?;
    fs::write("target/site/index.html", rendered)?;
    let theme_static_path = format!("{}/static", theme_path);
    let build_static_path = "target/site/static";

    if !Path::new(build_static_path).exists() {
        symlink(&theme_static_path, build_static_path)?;
    }

    println!("listening on 'http://localhost:3030'");

    warp::serve(warp::fs::dir("target/site"))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
