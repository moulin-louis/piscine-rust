#[allow(dead_code)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[allow(dead_code)]
impl Color {
	const WHITE: Self = Self {red:255, green:255, blue:255 };
	const RED: Self = Self {red:255, green:0, blue:0 };
	const GREEN: Self = Self {red:0, green:255, blue:0 };
	const BLUE: Self = Self {red:0, green:0, blue:255 };

	const fn new( red_i:u8, green_i:u8, blue_i:u8 ) -> Self {
		Self { red: red_i, green: green_i, blue: blue_i }
	}

	fn closest_mix( self, palette:&[(Self, u8)], max:u32 ) -> Self {
		self
	}
}


#[cfg(test)]
mod tests {
	use crate::Color;

	#[test]
	fn mandatory_test() {
		assert_eq!(Color::RED.closest_mix(&[], 100), Color::WHITE);
		assert_eq!(Color::RED.closest_mix(&[(Color::RED, 255)], 0), Color::WHITE);

		let palette = [(Color::RED, 100), (Color::GREEN, 100), (Color::BLUE, 100)];
		assert_eq!(
		Color::new(254, 23, 102).closest_mix(&palette, 5),
		Color::new(218, 20, 57),
);
	}
}
