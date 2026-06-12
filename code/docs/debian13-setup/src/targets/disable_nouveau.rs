use super::prelude::*;

/// Disable nouveau, the open-source nvidia drivers.
#[derive(FromArgs)]
#[argh(subcommand, name = "disable-nouveau")]
pub struct DisableNouveau {}

const CONTENTS: &str = "

blacklist nouveau
options nouveau modeset=0

";

const TARGET_FILE: &str = "/etc/modprobe.d/nouveau-blacklist.conf";

impl Target for DisableNouveau {
    fn install(&self) {
        utils::echo(CONTENTS.trim(), TARGET_FILE, true);
    }
}
