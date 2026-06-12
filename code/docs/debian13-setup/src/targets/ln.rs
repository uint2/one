use super::prelude::*;

/// Download and install ln.
#[derive(FromArgs)]
#[argh(subcommand, name = "ln")]
pub struct Ln {}

impl Target for Ln {
    fn install(&self) {
        let source_dir = dirs::REPOS_DIR.join("ln");
        pins::ln::REPO.ensure_exists_at(&source_dir);
        cmd!("make", "configure").current_dir(&source_dir).run().unwrap();
        cmd!("make", "build").current_dir(&source_dir).run().unwrap();
        cmd!("sudo", "make", "install").current_dir(&source_dir).run().unwrap();
    }
}
