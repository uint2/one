/// Represents a setup target for Debian.
pub trait Target {
    fn run(&self) {
        self.install()
    }

    fn install(&self);
    fn uninstall(&self) {
        println!("\x1b[33mNo uninstall script configured. No-op.\x1b[m");
    }
}
