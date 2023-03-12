pub fn print_bytes( s:&str ) {
	if s.is_empty() {
		return ;
	}
	for letter in s.as_bytes() {
		println!("{}", *letter as u8);
	}
}