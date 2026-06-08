mod cli;

use std::collections::HashSet;
use std::io::{BufReader, Read};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;
use std::{fs, fs::File, panic, process};

const KB: u64 = 1024;
const MB: u64 = 1024 * KB;
const GB: u64 = 1024 * MB;

/// Gets all files recursively.
fn get_all_files(path: &Path, files: &mut Vec<PathBuf>) {
    for ent in fs::read_dir(path).unwrap() {
        let ent = ent.unwrap();
        if ent.file_type().unwrap().is_dir() {
            get_all_files(&ent.path(), files);
        } else {
            files.push(ent.path())
        }
    }
}

fn main() {
    let args: cli::Cli = argh::from_env();

    let mut files0 = vec![];
    let mut files1 = vec![];

    get_all_files(args.left(), &mut files0);
    get_all_files(args.right(), &mut files1);
    files0.sort();
    files1.sort();

    let short_a: Vec<_> =
        files0.iter().map(|v| v.strip_prefix(args.left()).unwrap()).collect();
    let short_b: Vec<_> =
        files1.iter().map(|v| v.strip_prefix(args.right()).unwrap()).collect();
    if short_a != short_b {
        // Display the difference between the filenames of the two directories.
        let set_a: HashSet<&Path> = HashSet::from_iter(short_a);
        let set_b: HashSet<&Path> = HashSet::from_iter(short_b);
        let mut diff: Vec<_> = set_a.symmetric_difference(&set_b).collect();
        diff.sort();
        for file in diff {
            if set_a.contains(file) {
                println!("< : {}", file.display())
            } else {
                println!("> : {}", file.display())
            }
        }
        return;
    }

    println!("Directories contain the exact same filenames!");
    let n = files0.len();
    const N: usize = 0x1000;
    let pool = rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap();

    let counter = Arc::new(AtomicU32::new(0));

    let start_t = Instant::now();

    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    pool.scope(|s| {
        for i in 0..n {
            let short = &short_a[i];
            let f0 = &files0[i];
            let f1 = &files1[i];
            let counter = Arc::clone(&counter);
            s.spawn(move |_| {
                let j = counter.fetch_add(1, Ordering::Relaxed);
                println!("[{j}/{n}] {}", short.display());
                let m0 = f0.metadata().unwrap();
                let m1 = f1.metadata().unwrap();
                assert_eq!(m0.file_type(), m1.file_type());
                assert_eq!(m0.size(), m1.size());
                if m0.size() >= 1 * GB {
                    println!("Too big. Skipping file diff.");
                    return;
                }
                let mut r0 = BufReader::new(File::open(f0).unwrap());
                let mut r1 = BufReader::new(File::open(f1).unwrap());

                let mut buf0 = [0; N];
                let mut buf1 = [0; N];

                loop {
                    let out0 = match r0.read(&mut buf0) {
                        Ok(0) | Err(_) => break,
                        Ok(v) => v,
                    };
                    let out1 = match r1.read(&mut buf1) {
                        Ok(0) | Err(_) => break,
                        Ok(v) => v,
                    };
                    if out0 != out1 || buf0 != buf1 {
                        panic!("Mismatch: {}", short.display())
                    }
                }
                if buf0 != buf1 {
                    panic!("Mismatch: {}", short.display())
                }
            });
        }
    });
    let elapsed = start_t.elapsed();
    println!("Directories contain the exact same files!");
    println!("Elapsed: {elapsed:?}");
}
