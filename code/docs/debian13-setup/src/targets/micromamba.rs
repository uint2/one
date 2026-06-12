use super::prelude::*;

/// Download and install micromamba.
#[derive(FromArgs)]
#[argh(subcommand, name = "micromamba")]
pub struct Micromamba {}

impl Target for Micromamba {
    fn install(&self) {
        use pins::micromamba::DOWNLOAD_URL as URL;
        const OUTFILE: &str = "micromamba";

        utils::curl(OUTFILE, URL);
        let shasum = cmd!("wget", "--quiet", format!("{URL}.sha256"), "-O-")
            .collect_stdout()
            .run()
            .unwrap();
        assert_eq!(shasum.stdout.unwrap().trim(), utils::sha256sum(OUTFILE));
        sh!("chmod", "u+x", "micromamba");
        fs::rename("micromamba", dirs::HOME_DIR.join(".local/bin").join(OUTFILE))
            .unwrap();
        // Later on, run
        // micromamba create --name ml python=3.12 --channel=conda-forge
    }
}
