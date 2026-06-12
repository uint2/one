use super::prelude::*;

/// Download and install firefox.
#[derive(FromArgs)]
#[argh(subcommand, name = "firefox")]
pub struct Firefox {}

const GPG_KEY_URL: &str = "https://packages.mozilla.org/apt/repo-signing-key.gpg";
const GPG_KEY_PATH: &str = "/etc/apt/keyrings/packages.mozilla.org.asc";

impl Target for Firefox {
    /// Install firefox from their PPA repository.
    fn install(&self) {
        // Get the signing key.
        let output = cmd!("wget", "--quiet", GPG_KEY_URL, "-O-").collect_stdout().run();
        let signing_key = output.unwrap().stdout.expect("Unable to obtain signing key.");
        utils::echo(signing_key.trim(), GPG_KEY_PATH, true);
        // Add the newly added APT repository to our sources list:
        utils::echo(
            format!(
                "\
Types: deb
URIs: https://packages.mozilla.org/apt
Suites: mozilla
Components: main
Signed-By: {GPG_KEY_PATH}"
            )
            .trim(),
            "/etc/apt/sources.list.d/mozilla.sources",
            true,
        );
        // Configure APT to prioritize packages from the Mozilla repository:
        utils::echo(
            "\
Package: *
Pin: origin packages.mozilla.org
Pin-Priority: 1000"
                .trim(),
            "/etc/apt/preferences.d/mozilla",
            true,
        );

        sh!("sudo", "apt-get", "update", "--yes");
        sh!("sudo", "apt-get", "install", "--yes", "firefox");
    }
}
