fn increment (x: u8) -> u8 { x + 1 }

fn main() {
    let x = 255;
    println!("x: {}, x+1: {}", x, increment (x));
}