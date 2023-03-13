use ftkit::ARGS;
use ex07::strpcmp;

fn main() {
	if ARGS.len() != 3 {
		panic!("Wrong number of args");
	}
	if strpcmp(ARGS[1].as_bytes(), ARGS[2].as_bytes()) {
		println!("yes");
	}
	else {
		println!("no");
	}
}