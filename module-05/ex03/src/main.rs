use std::env::{args, args_os};
use std::io::{stdin, Read};
use std::process::Command;
use std::vec::Vec;

fn main() {
    let mut command = args().collect::<Vec<String>>();
    command.remove(0);
    if command.is_empty() {
        eprintln!("Wrong usage!");
        return;
    }
    let mut buff = Vec::new();
    if let Err(err) = stdin().read_to_end(&mut buff) {
        eprintln!("Error reading stdin: {}", err);
        return;
    }
    let str = match String::from_utf8(buff) {
        Err(err) => {
            eprintln!("Invalid char in stdin: {}", err);
            return;
        }
        Ok(val) => val,
    };
    let args = str.split('\n').collect::<Vec<&str>>();
    let mut cmd = Command::new(&command[0]);
    cmd.args(&command[1..command.len()]);
    cmd.args(args);
    match cmd.output() {
        Err(err) => {
            eprintln!("Failed to execute the command: {}", err);
        }
        Ok(val) => {
            let str = String::from_utf8(val.stdout).expect("CANT CONVERT OUTPUT TO STRING");
            print!("{}", str);
        }
    }
}
