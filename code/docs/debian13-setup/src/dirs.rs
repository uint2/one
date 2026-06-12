use lazy_static::lazy_static;

use std::path::{Path, PathBuf};
use std::{fs, io};

lazy_static! {
    pub static ref HOME_DIR: PathBuf = std::env::home_dir().unwrap();
    pub static ref REPOS_DIR: PathBuf = HOME_DIR.join("repos");
    pub static ref DIRS_TO_HAVE: [PathBuf; 3] =
        [HOME_DIR.join(".local"), HOME_DIR.join(".local/bin"), HOME_DIR.join("repos")];
}

/// Tries to create a directory. Succeeds upon success or if the directory
/// already exists.
pub fn try_create(path: &Path) -> io::Result<()> {
    let Err(err) = fs::create_dir(path) else { return Ok(()) };
    if let io::ErrorKind::AlreadyExists = err.kind() {
        return Ok(());
    }
    Err(err)
}
