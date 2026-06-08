use std::path::PathBuf;
use walkdir::WalkDir;

/// Number of components to display.
const N: usize = 4;

fn main() {
    let files = std::env::args_os().skip(1).flat_map(WalkDir::new);

    let mut abbrev_buf = PathBuf::with_capacity(N);

    for file in files {
        let Ok(file) = file else { continue };
        let pdf = match file.path().extension() {
            Some(v) if v == "pdf" => file.path(),
            _ => continue,
        };

        let components = pdf.components().collect::<Vec<_>>();
        let n = components.len();
        if n < N {
            println!("{}", pdf.display());
            continue;
        }
        // Beyond here, n ≥ N.
        abbrev_buf.clear();
        abbrev_buf.extend(&components[n - N..]);
        // https://davatorium.github.io/rofi/1.7.3/rofi-script.5/
        // This tells us that the `info` component shall be used to update
        // $ROFI_INFO.
        println!("{}\0info\x1f{}", abbrev_buf.display(), pdf.display());
    }
}
