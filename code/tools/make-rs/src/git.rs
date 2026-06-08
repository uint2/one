use crate::debug_tools::*;

use git2::build::CheckoutBuilder;
use git2::{Index, Repository};

use std::path::Path;
use std::{fs, io};

macro_rules! git {
    ($($arg:expr),*) => { std::process::Command::new("git")$(.arg($arg))* };
}

/// Git refs, in order of decreasing stability.
pub enum Ref {
    Sha(&'static str),
    Tag(&'static str),
    Branch(&'static str),
}

impl Ref {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Sha(v) | Self::Tag(v) | Self::Branch(v) => v,
        }
    }
}

pub struct Repo {
    pub url: &'static str,
    pub tag: Ref, // "ref" is already a Rust keyword.
}

pub fn is_in_git_worktree(dir: &Path) -> bool {
    let output = git!("-C", dir, "rev-parse", "--is-inside-work-tree")
        .output()
        .expect("Spawning `git rev-parse --is-inside-work-tree` failed.");
    output.status.success() && output.stdout.trim_ascii() == b"true"
}

pub fn count_remotes(dir: &Path) -> usize {
    let output =
        git!("-C", dir, "remote").output().expect("Spawning `git remote` failed.");
    if !output.status.success() {
        return 0;
    }
    output.stdout.iter().filter(|v| **v == b'\n').count()
}

pub fn rev_parse(dir: &Path, item: &str) -> String {
    let output = git!("-C", dir, "rev-parse", item)
        .output()
        .expect("Spawning `git rev-parse` failed.");
    s2(&output.stdout).to_string()
}

impl Repo {
    /// Clones this repo to a directory.
    /// TODO: optimize this with shallow clones where possible.
    pub fn clone_to_dir(&self, dir: &Path) {
        let mut child =
            git!("clone", self.url, dir).spawn().expect("Spawning `git clone` failed.");
        child.wait().expect("Waiting for child process failed.");
    }

    /// Tries to checkout `self.tag` at a particular directory.
    pub fn checkout_ref(&self, dir: &Path) {
        let mut child = git!("-C", dir, "checkout", self.tag.as_str())
            .spawn()
            .expect("Spawning `git checkout` failed.");
        child.wait().expect("Waiting for child process failed.");
    }

    pub fn fresh_clone_and_checkout(&self, dir: &Path) {
        let _ = fs::remove_dir_all(dir);
        let _ = fs::create_dir_all(dir.parent().expect("No parent?!"));
        self.clone_to_dir(dir);
        self.checkout_ref(dir);
    }

    /// Ensures that this repo exists at `dir`, and checked out to `self.tag`
    pub fn sync(&self, dir: &Path) {
        // Check if it's a git repo.
        if !is_in_git_worktree(dir) {
            self.fresh_clone_and_checkout(dir);
            return;
        }
        // TODO: check that there's only one remote.
        // TODO: check that the remote is "origin".

        // Check that the correct git ref is checked out.
        if rev_parse(dir, "HEAD") != rev_parse(dir, self.tag.as_str()) {
            self.checkout_ref(dir);
        }
        // TODO: check that the git status is clean.
    }
}
