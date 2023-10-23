use ex07::strpcmp;

use ftkit::ARGS;

fn main() {
    let argv = ARGS;
    if argv.len() != 3 {
        eprintln!("Wrong number of args!");
        return;
    }
    println!("{}", strpcmp(argv[1].as_bytes(), argv[2].as_bytes()));
}
