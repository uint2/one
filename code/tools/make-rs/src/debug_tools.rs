pub fn s(s: &str) -> String {
    String::from(s)
}

pub fn s2(s: &[u8]) -> &str {
    core::str::from_utf8(s).unwrap()
}

macro_rules! sh {
    ($cmd:expr, $($arg:expr),*) => { std::process::Command::new($cmd)$(.arg($arg))* };
}
