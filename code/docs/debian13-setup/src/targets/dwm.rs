use super::prelude::*;

/// Download and install dwm
#[derive(FromArgs)]
#[argh(subcommand, name = "dwm")]
pub struct Dwm {}

impl Target for Dwm {
    fn install(&self) {
        let source_dir = dirs::REPOS_DIR.join("dwm");
        pins::dwm::REPO.ensure_exists_at(&source_dir);
        cmd!("make", "configure").current_dir(&source_dir).run().unwrap();
        cmd!("make", "build").current_dir(&source_dir).run().unwrap();
        cmd!("sudo", "make", "install").current_dir(&source_dir).run().unwrap();
    }
}
