use std::println;

mod collatz;
use collatz::collatz;
mod print_bytes;
use print_bytes::print_bytes;

#[allow(dead_code)]
fn yes() -> ! {
    loop {
        println!("y");
    }
}

fn main() {
    println!("Collatz test:");
    collatz(3);
    println!("\nprint_bytes test:");
    print_bytes("Déjà Vu\n");
    // yes();
}
