fn largest_group<'a>( haystack:&'a [u32], needle:&[u32] ) -> &'a[u32] {
	if needle.len() > haystack.len() { panic!(); }
	if needle.is_empty() { return &[]; }
	for i in needle { if !haystack.contains(i) { return &[]; } }

	let mut start:usize = 0;
	let mut end:usize = 0;
	let mut index:usize = 0;
	for i in haystack {
		if !needle.contains(i) {
			index += 1;
			continue ;
		}

		println!("found {} at pos {}", i, index);

		let temp_start = index;
		let mut it = index;

		while it < haystack.len(){
			if !needle.contains(&haystack[it]) {
				break ;
			}
			it += 1;
		}

		let temp_end = it;

		println!("temp s = {}, temp e = {}", temp_start, temp_end);

		if temp_end - temp_start > end - start {
			end = temp_end;
			start = temp_start;
		}
		index += 1;
	}
	if end - start < needle.len() {
		return &[];
	}
	println!("start = {}, end = {}", start, end);
	&haystack[start..end]
}

#[cfg(test)]
mod tests {
	use crate::largest_group;

	#[test]
	fn test_lifetimes() {
		let haystack = [1, 2, 3, 2, 1];
		let result;

		{
			let needle = [2, 3];
			result = largest_group(&haystack, &needle);
		}

		assert_eq!(result, &[2, 3, 2]);
	}

	#[test]
	fn mandatory_test_1() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);   
	}
	#[test]
	fn mandatory_test_2() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
	}
	#[test]
	fn mandatory_test_3() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
	}
	#[test]
	fn mandatory_test_4() {
		assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
	}
	
}