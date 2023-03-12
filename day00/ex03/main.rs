fn main() {
	for i in 1..101 {
		let mut nl = 0;
		if i % 3 == 0 {
			print!("fizz");
			nl = 1;
		}
		if i % 5 == 0 {
			print!("buzz");
			nl = 1;
		}
		if nl == 1 {
			print!("\n");
			continue;
		}
		if i % 11 == 3 % 11 {
			print!("FIZZ\n");
			continue ;
		}
		if i % 11 == 5 % 11 {
			print!("BUZZ\n");
			continue ;
		}
		print!("{}\n", i);
	}
}