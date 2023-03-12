pub fn collatz( start:u32 ) {
	let mut temp:u32 = start;
	println!("{}", temp);
	
	while temp != 1 {
		if (temp & 1) == 0 {
			temp /= 2;
			println!("{}", temp);
		}
		else {
			temp *= 3;
			temp += 1;
			println!("{}", temp);
		}
	}
}