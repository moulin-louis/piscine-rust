use std::ops::FnMut;

mod print_bytes;
mod yes;

use crate::print_bytes::print_bytes;
use crate::yes::yes;

fn collatz<F: FnMut(u32)>(start: u32, mut callback: F) {
    for idx in 0..start {
        if idx % 2 != 0 {
            callback(idx);
        }
    }
}

fn main() {
    collatz(10, |x| println!("{}", x));
    // print_bytes(|| Some(b'a'));
    let string = String::from("y");
    let function = || string;
    yes(function);
}

#[cfg(test)]
mod tests {
    use crate::collatz;
    use crate::print_bytes::*;
    use crate::yes::*;

    #[test]
    fn test_collatz() {
        let mut vec = Vec::new();
        collatz(10, |x| vec.push(x));
        assert_eq!(vec, vec![1, 3, 5, 7, 9]);
    }
}
