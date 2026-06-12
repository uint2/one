use super::prelude::*;

/// Download and install neovim.
#[derive(FromArgs)]
#[argh(subcommand, name = "neovim")]
pub struct Neovim {}

impl Target for Neovim {
    fn install(&self) {
        let source_dir = dirs::REPOS_DIR.join("neovim");
        let build_dir = source_dir.join("build");

        // Clone the repo.
        let repo = GitRepo::new(pins::neovim::REMOTE_URL, pins::neovim::TAG.as_str());
        repo.ensure_exists_at(&source_dir);

        // Build neovim.
        cmd!("make", "CMAKE_BUILD_TYPE=Release")
            .current_dir(source_dir)
            .stdout("nvim-build.stdout.txt")
            .stderr("nvim-build.stderr.txt")
            .run()
            .unwrap();

        // Remove existing *.deb files.
        utils::search_dir(&build_dir, |v| utils::has_ext(v.path(), "deb"))
            .iter()
            .for_each(|v| fs::remove_file(v).unwrap());

        // Create a .deb package with the build outputs.
        cmd!("cpack", "-G", "DEB").current_dir(&build_dir).run().unwrap();

        let deb_files =
            utils::search_dir(&build_dir, |v| utils::has_ext(v.path(), "deb"));
        cmd!("sudo", "dpkg", "--install", &deb_files[0])
            .current_dir(&build_dir)
            .run()
            .unwrap();
    }
}
