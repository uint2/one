use super::prelude::*;

/// Download and install zls, the zig LSP.
/// Note that this requires `minisign` to be installed.
#[derive(FromArgs)]
#[argh(subcommand, name = "zls")]
pub struct Zls {}

impl Target for Zls {
    fn install(&self) {
        const SIGNATURE_FILE: &str = "/tmp/.minisig";

        let destination = dirs::HOME_DIR.join(".local/bin/zls");

        let _ = fs::remove_file(&destination);

        utils::curl(&destination, pins::zls::SOURCE_URL);
        utils::curl(SIGNATURE_FILE, &format!("{}.minisig", pins::zls::SOURCE_URL));
        sh!(
            "minisign",
            "-Vm",
            &destination,
            "-P",
            pins::zls::MINISIGN_SIGNATURE,
            "-x",
            SIGNATURE_FILE
        );

        sh!("chmod", "u+x", &destination);

        let _ = fs::remove_file(SIGNATURE_FILE);
    }
}
