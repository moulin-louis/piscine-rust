use std::println;

fn min(a: i32, b: i32) -> i32 {
    let result:i32;
    if a < b {
        result = a;
    } else {
        result = b;
    }
    result
}

fn main() {
    println!("min({}, {}) = {}", 1, 2, min(1, 2));
    println!("min({}, {}) = {}", 2, 1, min(2, 1));
    println!("min({}, {}) = {}", -1, 1, min(-1, 1));
    println!("min({}, {}) = {}", 1, -1, min(1, -1));
    println!("min({}, {}) = {}", 0, 0, min(0, 0));
    println!("min({}, {}) = {}", 0, 1, min(0, 1));
    println!("min({}, {}) = {}", 1, 0, min(1, 0));
    println!("min({}, {}) = {}", -1, 0, min(-1, 0));
}