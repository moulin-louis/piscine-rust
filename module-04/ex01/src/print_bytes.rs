use std::ops::Fn;

#[allow(dead_code)]
pub fn print_bytes<F: Fn() -> Option<u8>>(function: F) {
    while let Some(x) = function() {
        for bits in 0..8 {
            if x & (1 << bits) != 0 {
                print!("Y");
            } else {
                print!("y");
            }
        }
        println!();
    }
}
