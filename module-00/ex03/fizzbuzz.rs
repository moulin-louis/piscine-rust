use std::println;

fn main() {
    for num in 1..=100 {
        let output = match (num % 3, num % 5, num % 11) {
            (0, 0, _) => "fizzbuzz".to_string(),
            (0, _, _) => "fizz".to_string(),
            (_, 0, _) => "buzz".to_string(),
            (_, _, 3) => "FIZZ".to_string(),
            (_, _, 5) => "BUZZ".to_string(),
            _ => num.to_string(),
        };
        println!("{}", output);
    }
}