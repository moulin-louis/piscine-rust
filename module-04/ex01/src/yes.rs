use std::ops::FnOnce;

pub fn yes<F: FnOnce() -> String>(callback: F) {
    let result: String = callback();
    loop {
        println!("{}", result);
    }
}
