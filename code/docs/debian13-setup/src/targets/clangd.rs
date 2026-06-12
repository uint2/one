use super::prelude::*;

/// Download and install clangd.
#[derive(FromArgs)]
#[argh(subcommand, name = "clangd")]
pub struct Clangd {}

const VERSION: &str = "22.1.0";

/// Download URL for Clangd.
fn download_url(tag: &str) -> String {
    format!(
        "https://github.com/clangd/clangd/releases/download/{tag}/clangd-linux-{tag}.zip"
    )
}

impl Target for Clangd {
    fn install(&self) {
        let download_url = download_url(VERSION);
        let extracted_dir = PathBuf::from(format!("clangd_{VERSION}"));
        let zip_file = extracted_dir.with_extension("zip");

        let _ = fs::remove_file(&zip_file);
        let _ = fs::remove_dir_all(&extracted_dir);

        utils::curl(&zip_file, &download_url);
        sh!("unzip", &zip_file);
        sh!("sudo", "rm", "-f", "/usr/local/bin/clangd");
        sh!("sudo", "mv", extracted_dir.join("bin/clangd"), "/usr/local/bin/clangd");

        let _ = fs::remove_file(&zip_file);
        let _ = fs::remove_dir_all(&extracted_dir);
    }
}
