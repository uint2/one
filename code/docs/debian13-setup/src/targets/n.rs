use super::prelude::*;

/// Download and install n.
/// https://github.com/tj/n
#[derive(FromArgs)]
#[argh(subcommand, name = "n")]
pub struct NTheNodeVersionManager {}

/// Prefixed upon the home directory.
const N_PREFIX: &str = ".local/n";

impl Target for NTheNodeVersionManager {
    fn install(&self) {
        const INSTALLER_PATH: &str = "n-install.sh";
        let _ = fs::remove_file(INSTALLER_PATH);
        utils::curl(INSTALLER_PATH, pins::n::SOURCE_URL);
        let n_prefix = dirs::HOME_DIR.join(N_PREFIX);
        cmd!("bash", INSTALLER_PATH, "install", "lts")
            .env("N_PREFIX", &n_prefix)
            .run()
            .unwrap();

        if let pins::neovim::Tag::V0_11_7 = pins::neovim::TAG {
            let path = env::var("PATH").unwrap();
            // Install this so that neovim can use it.
            cmd!("npm", "install", "-g", "tree-sitter-cli@0.25.10")
                .env("N_PREFIX", &n_prefix)
                .env("PATH", format!("{}:{path}", n_prefix.display()))
                .run()
                .unwrap();
        }
    }
}
