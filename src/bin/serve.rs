use std::env;

use wmrio::{content, path_manager::PathManager, render};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let site = content::parse_site()?;
    render::site(&site)?;

    // serve up site
    println!("listening on 'http://localhost:3030'");
    let pm = PathManager::from_env()?;
    env::set_current_dir(&pm.project_root)?;

    warp::serve(warp::fs::dir("target/site"))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
