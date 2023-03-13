fn main() {
	let answers = ftkit::random_number(0..5000);
	println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for.");
	loop {
		let input = ftkit::read_number();
		if input < answers {
			println!("This student might not be as smart as I was told. This answer is obviously too weak.");
		}
		if input > answers {
			println!("Sometimes I wonder whether I should retire. I would have guessed higher.");
		}
		else {
			println!("That is right! The secret was indeed the number {}, which you have brilliantly discovered!", answers);
			break ;
		}
	}
}
