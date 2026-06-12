use super::prelude::*;

/// Download and install less.
#[derive(FromArgs)]
#[argh(subcommand, name = "less")]
pub struct Less {}

impl Target for Less {
    fn install(&self) {
        let source_dir = dirs::REPOS_DIR.join("less");
        pins::less::REPO.ensure_exists_at(&source_dir);
        cmd!("make", "-f", "Makefile.aut", "distfiles")
            .current_dir(&source_dir)
            .run()
            .unwrap();
        cmd!("sh", "configure").current_dir(&source_dir).run().unwrap();
        cmd!("make").current_dir(&source_dir).run().unwrap();
        cmd!("sudo", "make", "install").current_dir(&source_dir).run().unwrap();
    }
}
