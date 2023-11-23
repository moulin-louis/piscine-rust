use std::env::args;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

fn create_write_file(name: String, content: &[u8]) {
    let mut file = match File::create("./".to_owned() + &*name) {
        Err(err) => {
            eprintln!("Failed to create {}: {}", name, err);
            return;
        }
        Ok(val) => val,
    };
    if let Err(err) = file.write_all(content) {
        eprintln!("Failed to write content to {}: {}", name, err);
    };
}

fn main() {
    let mut files_out: Vec<String> = args().rev().collect();
    files_out.pop();
    let mut input: Vec<u8> = Vec::new();
    match stdin().read_to_end(&mut input) {
        Err(err) => {
            eprintln!("Error reading stdin: {}", err);
            return;
        }
        Ok(_nbr_read) => {}
    };
    match stdout().write(input.as_slice()) {
        Err(err) => {
            eprintln!("Error writting to stdout: {}", err);
            return;
        }
        Ok(_nbr_write) => {}
    };
    for file in files_out {
        create_write_file(file, input.as_slice());
    }
}
