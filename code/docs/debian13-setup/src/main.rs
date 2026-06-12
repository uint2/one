//! Setup Debian 13 the way that Khang sets up his workspace.
//!
//! This requires "sudo" to already be installed.

#[macro_use]
mod macros;
#[macro_use]
mod command;
mod dirs;
mod git;
mod pins;
mod target;
mod targets;
mod utils;

fn main() {
    use target::Target;

    let opts = targets::parse_cli();

    for dir in &*dirs::DIRS_TO_HAVE {
        dirs::try_create(dir).unwrap();
    }

    opts.target.run();
}
