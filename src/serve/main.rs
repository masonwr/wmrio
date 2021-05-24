use std::{env, fs};
use tera::{Context, Tera};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project_root = env::var("CARGO_MANIFEST_DIR")?;
    // todo cd to project root

    let template_path = format!("{}/templates/default/**/*.html", project_root);

    let tera = Tera::new(&template_path)?;

    let mut context = Context::new();
    context.insert("title", "wmrio");

    let rendered = tera.render("index.html", &context)?;

    println!("{:?}", rendered);

    fs::create_dir_all("target/site")?;
    fs::write("target/site/index.html", rendered)?;

    // write out, and then server

    warp::serve(warp::fs::dir("target/site"))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
