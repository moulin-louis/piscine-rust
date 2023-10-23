use ftkit::{ARGS, read_line};
use unicode_width::UnicodeWidthStr;

fn main() {
    if ARGS.len() != 2 {
        panic!("Wrong nbr of argument!");
    }
    let nbr_column: u32 = match ARGS[1].parse() {
        Err(e) => panic!("{e}"),
        Ok(t) => t,
    };
    loop {
        let line = read_line();
    }
    println!("arg = {nbr_column}");
}
