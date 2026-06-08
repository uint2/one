use argh::FromArgs;

use std::path::Path;

#[derive(FromArgs, Debug)]
/// Top-level CLI.
pub struct Cli {
    #[argh(positional)]
    left: String,

    #[argh(positional)]
    right: String,
}

impl Cli {
    pub fn left(&self) -> &Path {
        Path::new(self.left.as_str())
    }

    pub fn right(&self) -> &Path {
        Path::new(self.right.as_str())
    }
}
