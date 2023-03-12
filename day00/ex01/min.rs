pub fn min( a: i32, b: i32 ) -> i32 {
	if a < b {
		a
	}
	else {
		b
	}
}

fn main() {
	let a:i32 = 1;
	let mut b:i32 = 0;
	println!("min is {}", min(a, b));
	b = 2;
	println!("min is {}", min(a, b));
}