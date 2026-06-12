use super::prelude::*;

/// Download and setup qmk.
#[derive(FromArgs)]
#[argh(subcommand, name = "qmk")]
pub struct Qmk {}

impl Target for Qmk {
    fn install(&self) {
        const INSTALLER: &str = "qmk-install.sh";
        utils::curl(INSTALLER, "https://install.qmk.fm");
        sh!("sh", INSTALLER, "--confirm", "--skip-package-manager");
        let _ = fs::remove_file(INSTALLER);
    }
}
