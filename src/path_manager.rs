use std::env;
use std::path::PathBuf;

pub struct PathManager {
    pub project_root: String,
}

// TODO: convert this to a set of functions backed by lazy static
impl PathManager {
    pub fn from_env() -> anyhow::Result<Self> {
        let project_root = env::var("CARGO_MANIFEST_DIR")?;
        Ok(PathManager { project_root })
    }

    pub fn theme_root(&self) -> PathBuf {
        let mut theme_root = PathBuf::from(&self.project_root);
        theme_root.push("themes/default");

        theme_root
    }

    pub fn theme_static_path(&self) -> PathBuf {
        let mut path = self.theme_root();
        path.push("static");
        path
    }

    pub fn out_path(&self) -> PathBuf {
        let mut out_path = PathBuf::from(&self.project_root);
        out_path.push("target/site");
        out_path
    }

    pub fn out_static_path(&self) -> PathBuf {
        let mut p = self.out_path();
        p.push("static");
        p
    }
}
