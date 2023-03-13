#[allow(dead_code)]
fn convert_i32( nbr:&[u8] ) -> u32 {
	let mut result: u32 = 0;
	for i in nbr {
		if *i == 0 {
			continue;
		}
		else {
			result *= 10;
			result += ((*i) - 48) as u32;
		}
	}
	println!("result = {}", result);
	result
}

#[allow(dead_code)]
fn len_result_nbr( mut nbr: u32 ) ->  u8 {
	let mut result:u8 = 0;
	while nbr > 1 {
		nbr /= 10;
		result += 1;
	}
	if nbr == 1 {
		result += 1;
	}
	result
}

#[allow(dead_code)]
fn big_add( a:&[u8], b:&[u8] ) -> Vec<u8> {
	if a.is_empty() { panic!(); }
	if b.is_empty() { panic!(); }

	let mut result: Vec<u8> = Vec::new();

	let mut result_nbr = convert_i32(a) + convert_i32(b);
	println!("result_nbr = {}", result_nbr);
	let mut len = len_result_nbr(result_nbr);
	println!("len = {}", len);
	while len != 0 {
		result.push((result_nbr % 10) as u8  + 48);
		result_nbr /= 10;
		len -= 1;
	}
	result.reverse();
	result
}

#[cfg(test)]
mod tests {
	use crate::big_add;

	#[test]
	fn mandatory_test_1() {
		assert_eq!(big_add(b"2", b"4"), b"6");
	}
	#[test]
	fn mandatory_test_2() {
		assert_eq!(big_add(b"0010", b"0200"), b"210");
	}
}
