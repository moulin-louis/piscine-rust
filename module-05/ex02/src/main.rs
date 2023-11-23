use std::env::args;
use std::fs::{metadata, read_dir, DirEntry};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

fn print_current_size(current_size: u128) {
    println!("current_size = {}", current_size);
    let kilobyte: u128 = 1000;
    let megabyte: u128 = kilobyte * 1000;
    let gigabyte: u128 = megabyte * 1000;

    if current_size < kilobyte {
        println!("{} bytes", current_size);
    } else if current_size < megabyte {
        println!("{:.1} kilobytes", current_size as f64 / kilobyte as f64);
    } else if current_size < gigabyte {
        println!("{:.1} megabytes", current_size as f64 / megabyte as f64);
    } else {
        println!("{:.1} gigabytes", current_size as f64 / gigabyte as f64);
    }
}

fn fetch_size_entry(dir: &DirEntry) -> u128 {
    println!("reading {:?}", dir.path());
    match metadata(dir.path()) {
        Err(_) => {
            // eprintln!("Can fetch size of {:?} : {}", dir, err);
            0
        }
        Ok(val) => {
            println!("size of this file is {}", val.size());
            val.size() as u128
        }
    }
}

fn fetch_all_dir(path: &Path, mut current_size: u128) -> u128 {
    let dirs = match read_dir(path) {
        Err(_) => {
            // eprintln!("Error opening {:?}: {}", path, err);
            return current_size;
        }
        Ok(val) => val,
    };
    for entry in dirs {
        match entry {
            Err(_) => {
                // eprintln!("Cant read dir: {}", err);
                return current_size;
            }
            Ok(val) => {
                current_size += fetch_size_entry(&val);
                print_current_size(current_size);
                current_size = fetch_all_dir(val.path().as_path(), current_size);
            }
        };
    }
    current_size
}

fn main() {
    let mut args: Vec<String> = args().rev().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of args!");
        return;
    }
    args.pop();
    let path_dir = Path::new(&args[0]);
    let size = fetch_all_dir(path_dir, 0);
    print_current_size(size);
}
