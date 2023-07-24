use std::println;

pub fn print_bytes(s: &str) {
    for b in s.bytes() {
        println!("{}", b);
    }
}