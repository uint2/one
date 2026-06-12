use super::prelude::*;

/// Download and install Yubikey.
/// Gets my public key from github, and then tells gpg of the presence of the
/// private key on the currently plugged-in hardware key.
#[derive(FromArgs)]
#[argh(subcommand, name = "yubikey")]
pub struct Yubikey {}

const TMP_PUB_KEY_PATH: &str = "d1b27668f89f3894b16225ce8f609dc6db350697.asc";

impl Target for Yubikey {
    fn install(&self) {
        utils::curl(TMP_PUB_KEY_PATH, "https://github.com/nguyenvukhang.gpg");
        sh!("gpg", "--import", TMP_PUB_KEY_PATH);
        let _ = fs::remove_file(TMP_PUB_KEY_PATH);

        sh!("gpg", "--card-status");
        sh!("gpg", "--list-secret-keys");

        println!("Remember to use `gpg --edit-keys` and trust your own key!");
    }
}
