#![allow(unused)]

#[macro_use]
mod git;
#[macro_use]
mod debug_tools;

mod consts;
mod enums;

use debug_tools::*;
use enums::*;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

struct Target {
    /// Name of the compile target. Cannot be empty.
    /// TODO: Replace this with a Name struct that enforces non-empty.
    name: &'static str,
    /// TODO: make this into an enum.
    compiler: &'static str,
    /// TODO: make this into an enum.
    language_standard: Option<&'static str>,
    include_dirs: Vec<&'static str>,
    flags: Vec<&'static str>,
    compile_definitions: Vec<CompileDefinition>,
    sources: Vec<&'static str>,
    git_repo: Option<git::Repo>,
}

impl Target {}

/*
/usr/bin/cc
-I/home/khang/repos/cwalk/include
-std=gnu11
-Wall
-Wextra
-Wpedantic
-Werror
-o
CMakeFiles/cwalk.dir/src/cwalk.c.o
-c
/home/khang/repos/cwalk/src/cwalk.c
*/

fn build(t: Target) {
    let srcdir = Path::new(consts::dir::build::deps::SOURCE).join(t.name);
    let builddir = Path::new(consts::dir::build::deps::BUILD).join(t.name);
    if let Some(repo) = t.git_repo {
        repo.sync(&srcdir);
        println!("[\x1b[36m{}\x1b[m] Sync complete", t.name);
    }
    for source in t.sources {
        let mut cmd = Command::new(t.compiler);
        if let Some(l_std) = t.language_standard {
            cmd.arg(format!("-std={l_std}"));
        }
        let srcpath = srcdir.join(source);
        let filename = srcpath.file_name().expect("file_name() failed.");
        let buildpath = builddir.join(filename).with_added_extension("o");
        cmd.arg("-o").arg(&buildpath);
        cmd.arg("-c").arg(&srcpath);
        for incl in &t.include_dirs {
            cmd.arg("-I").arg(srcdir.join(incl));
        }
        println!("{:?}", cmd);
        let _ = fs::create_dir_all(buildpath.parent().unwrap());
        let mut child = cmd.spawn().expect("Spawning compiler failed");
        child.wait().expect("Waiting for compiler child process failed.");
    }
}

fn main() {
    let cwalk = Target {
        name: "cwalk",
        language_standard: Some("gnu11"),
        compiler: "/usr/bin/cc",
        include_dirs: vec!["include"],
        flags: vec![],
        compile_definitions: vec![],
        sources: vec!["src/cwalk.c"],
        git_repo: Some(git::Repo {
            url: "https://github.com/likle/cwalk.git",
            tag: git::Ref::Sha("f45a23a13abf39d94b347d7c83810eca26a5a8d0"),
        }),
    };
    build(cwalk);
}
