use super::prelude::*;

use pins::nvidia::Version;

/// Download and install NVIDIA drivers.
#[derive(FromArgs)]
#[argh(subcommand, name = "nvidia")]
pub struct Nvidia {
    #[argh(subcommand)]
    pub subaction: Subaction,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Subaction {
    Install(Install),
    Uninstall(Uninstall),
}

/// Install the drivers.
#[derive(FromArgs)]
#[argh(subcommand, name = "install")]
pub struct Install {}

/// Uninstall the drivers.
#[derive(FromArgs)]
#[argh(subcommand, name = "uninstall")]
pub struct Uninstall {}

const SCRIPT_PATH: &str = "nvidia-install.sh";
const STANDARD_ARGS: [&str; 2] = ["--ui=none", "--rebuild-initramfs"];

/// Downloads the install script if it doesn't exist.
fn ensure_script_exists(version: Version) {
    if utils::get_file_size_with_du(SCRIPT_PATH) == version.num_bytes() {
        return println!("Skip the download!");
    }
    // Run the download.
    utils::curl(SCRIPT_PATH, &version.download_url());
}

impl Target for Nvidia {
    fn run(&self) {
        match self.subaction {
            Subaction::Install(_) => self.install(),
            Subaction::Uninstall(_) => self.uninstall(),
        }
    }

    fn install(&self) {
        ensure_script_exists(pins::nvidia::VERSION);
        let mut cmd = cmd!("sudo", "sh", SCRIPT_PATH);
        cmd.args(STANDARD_ARGS);
        cmd.arg("--no-questions");
        // Use MIT/GPL drivers instead of NVIDIA proprietary.
        cmd.args(["-M", "open"]);
        cmd.run().unwrap();
    }

    fn uninstall(&self) {
        ensure_script_exists(pins::nvidia::VERSION);
        let mut cmd = cmd!("sudo", "sh", SCRIPT_PATH);
        cmd.args(STANDARD_ARGS);
        cmd.arg("--uninstall");
        cmd.run().unwrap();
    }
}
