use std::env::args;
use std::io::{stderr, stdout, Write};
use std::process::{Child, Command};

#[derive(Debug)]
struct CmdChild {
    cmd: String,
    args: Vec<String>,
    pid: Child,
}

fn parse_cmd(args: &Vec<String>) -> Option<Vec<CmdChild>> {
    let mut result: Vec<CmdChild> = Vec::new();
    let mut tmp_command = String::new();
    let mut tmp_args: Vec<String> = Vec::new();
    for arg in args {
        if arg == &','.to_string() {
            result.push(CmdChild {
                cmd: tmp_command.clone(),
                args: tmp_args.clone(),
                pid: Command::new(tmp_command.clone())
                    .args(tmp_args.clone())
                    .spawn()
                    .expect("CANT SPAWN CHILD"),
            });
            tmp_command.clear();
            tmp_args.clear();
            continue;
        }
        if tmp_command.is_empty() {
            tmp_command = arg.clone();
            continue;
        }
        tmp_args.push(arg.clone());
    }
    result.push(CmdChild {
        cmd: tmp_command.clone(),
        args: tmp_args.clone(),
        pid: Command::new(tmp_command.clone())
            .args(tmp_args.clone())
            .spawn()
            .expect("CANT SPAWN CHILDREN"),
    });
    Some(result)
}

fn main() {
    let mut args = args().collect::<Vec<String>>();
    args.remove(0);
    if args.is_empty() {
        eprintln!("Wrong usage!");
        return;
    }
    let mut cmds: Vec<CmdChild> = match parse_cmd(&args) {
        Some(val) => val,
        None => {
            eprintln!("Cant parse args to commands!");
            return;
        }
    };
    loop {
        let mut to_handle: Vec<&CmdChild> = Vec::new();
        for process in &mut cmds {
            match process.pid.try_wait() {
                Ok(Some(_)) => {
                    to_handle.push(&process);
                }
                Ok(None) => {
                    println!("Get Ok(Non) from try wait");
                    stdout().flush().unwrap();
                }
                Err(err) => {
                    eprintln!("Error try to check wait status: {}", err);
                    stderr().flush().unwrap();
                    return;
                }
            }
        }
        for process in to_handle {
            let truc = &process.pid;
            match String::from_utf8(truc.wait_with_output().unwrap().stdout) {
                Err(err) => {
                    eprintln!("Error converting output to string: {}", err);
                    stderr().flush().unwrap();
                    return;
                }
                Ok(string) => {
                    // println!("==== {} {:?} ====", process.cmd, process.args);
                    print!("{}", string);
                    stdout().flush().unwrap();
                }
            }
        }
        if cmds.is_empty() {
            break;
        }
    }
}
