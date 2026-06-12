use super::prelude::*;

/// Download and install minisign.
#[derive(FromArgs)]
#[argh(subcommand, name = "minisign")]
pub struct Minisign {}

impl Target for Minisign {
    fn install(&self) {
        use pins::minisign::DOWNLOAD_URL as URL;

        const OUTFILE: &str = "minisign.tar.gz";
        const INSTALL_LOC: &str = "/usr/local/bin/minisign";

        utils::curl(OUTFILE, URL);

        sh!("tar", "-xvf", OUTFILE, "minisign-linux/x86_64/minisign");
        sh!("sudo", "rm", "-f", INSTALL_LOC);
        sh!("sudo", "cp", "minisign-linux/x86_64/minisign", INSTALL_LOC);
        let _ = fs::remove_dir_all("minisign-linux");
        let _ = fs::remove_dir_all(OUTFILE);
    }
}
