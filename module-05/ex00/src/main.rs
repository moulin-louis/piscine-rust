use std::io::{stdout, Write};

fn main() {
    for i in 1..10 {
        match stdout().write(i.to_string().as_bytes()) {
            Ok(_x) => {}
            Err(_e) => {}
        };
        match stdout().write('\n'.to_string().as_bytes()) {
            Ok(_x) => {}
            Err(_e) => {}
        };
    }
}
