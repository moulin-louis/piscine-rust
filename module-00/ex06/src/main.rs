use ftkit::{random_number, read_number};
use std::cmp::Ordering;

fn main() {
    let final_result: i32 = random_number(0..100);
    println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
    loop {
        let input: i32 = read_number();
        match input.cmp(&final_result) {
            Ordering::Less => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
            Ordering::Greater => println!("This student might not be as smart as I was told. This answer is obviously too weak."),
            Ordering::Equal => {
                println!("That is right! The secret was indeed the number {}, which you have brilliantly discovered!", final_result);
                break;
            }
        }
    }
}
