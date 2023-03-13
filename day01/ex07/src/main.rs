use ftkit::{ARGS, read_line};

fn main() {
    let args:Vec<&str> = ARGS.into_iter().collect();
    if args.len() != 2 {
        panic!("Wrong nnbr of args");
    }
    println!("{:?}", args);
    let user_input = read_line();
    println!("input = {}", user_input);
}
