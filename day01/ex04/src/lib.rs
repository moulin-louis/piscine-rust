fn is_sorted( boxes:&mut [[u32;2]] ) -> bool {
	for i in 0..(boxes.len() - 1) {
		if boxes[i][0] < boxes[i + 1][0] || boxes[i][1] < boxes[i + 1][1] {
			return false;
		}
	}
	true
}

fn sort_boxes( boxes:&mut [[u32; 2]]) {
	if boxes.is_empty() {
		panic!();
	}
	loop {
		println!("one loop");
		if is_sorted(boxes) {
			break ;
		}
		for i in 0..(boxes.len() - 1) {
			if boxes[i][0] < boxes[i + 1][0] || boxes[i][1] < boxes[i + 1][1] {
				boxes.swap(i, i + 1);
			}
		}
	}
}

#[cfg(test)]
mod test {
	use crate::sort_boxes;

	#[test]
	fn mandatory_test() {
		let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
	}

	#[test]
	fn additional_test_1() {
		let mut boxes = [[10,8], [4, 3], [120, 45], [5, 7], [3, 3]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[120, 45], [10, 8], [5, 7], [4, 3], [3, 3]]);
	}
}
