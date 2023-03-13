const fn color_name<'a>(color: &[u8; 3]) -> &'a str {
	let red = color[0];
	let green = color[1];
	let blue = color[2];
	if red == 0 && green == 0 && blue == 0 { return &"pure black"; }
	if red == 255 && green == 255 && blue == 255 { return &"pure white"; }
	if red == 255 && green == 0 && blue == 0 { return &"pure red"; }
	if red == 0 && green == 255 && blue == 0 { return &"pure green"; }
	if red == 0 && green == 0 && blue == 255 { return &"pure blue"; }
	if red < 31 && green < 31 && blue < 31 { return &"almost black"; }
	if red >= 128 && green <= 128 && blue <= 128 { return &"redish"; }
	if red <= 128 && green >= 128 && blue <= 128 { return &"greenish"; }
	if red <= 128 && green <= 128 && blue >= 128 { return &"blueish"; }
	return &"unknown";
}

#[cfg(test)]
mod tests {
	use crate::color_name;

	#[test]
	fn test_lifetimes() {
		let name_of_the_best_color;
	
		{
			let the_best_color = [42, 42, 42];
			name_of_the_best_color = color_name(&the_best_color);
		}
	
		assert_eq!(name_of_the_best_color, "unknown");
	}

	#[test]
	fn test_almost_black() {
		assert_eq!(color_name(&[30, 30, 30]), "almost black");
		assert_eq!(color_name(&[1, 1, 1]), "almost black");
	}

	#[test]
	fn test_pure_black() {
		assert_eq!(color_name(&[0,0,0]), "pure black");
	}

	#[test]
	fn test_pure_white() {
		assert_eq!(color_name(&[255,255,255]), "pure white");
	}

	#[test]
	fn test_pure_red() {
		assert_eq!(color_name(&[255,0,0]), "pure red");
	}

	#[test]
	fn test_pure_green() {
		assert_eq!(color_name(&[0,255,0]), "pure green");
	}

	#[test]
	fn test_pure_blue() {
		assert_eq!(color_name(&[0,0,255]), "pure blue");
	}

	#[test]
	fn test_redish() {
		assert_eq!(color_name(&[130,50,50]), "redish");
		assert_eq!(color_name(&[130,127,127]), "redish");
		assert_eq!(color_name(&[138,127,127]), "redish");
	}

	#[test]
	fn test_greenish() {
		assert_eq!(color_name(&[50,130,50]), "greenish");
		assert_eq!(color_name(&[127,130,127]), "greenish");
		assert_eq!(color_name(&[127,138,127]), "greenish");
	}

	#[test]
	fn test_blueish() {
		assert_eq!(color_name(&[50,50,130]), "blueish");
		assert_eq!(color_name(&[127,127,130]), "blueish");
		assert_eq!(color_name(&[127,127,138]), "blueish");
	}
}
