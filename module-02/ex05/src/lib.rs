#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[allow(dead_code)]
impl Color {
    const WHITE: Self = Color {
        red: 255,
        green: 255,
        blue: 255,
    };
    const RED: Self = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    const GREEN: Self = Color {
        red: 0,
        green: 255,
        blue: 0,
    };
    const BLUE: Self = Color {
        red: 0,
        green: 0,
        blue: 255,
    };

    const fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    fn normalize_alpha(alpha: u8) -> u8 {
        (alpha - u8::MIN) / (u8::MAX - u8::MIN)
    }

    fn color_blending(color_a: Self, alpha: u8, color_b: Self) -> Self {
        let alpha: u8 = Color::normalize_alpha(alpha);
        let red: u32 =
            ((color_a.red as u32) * alpha as u32) + (color_b.red as u32) * (1 - alpha as u32);
        let green: u32 =
            ((color_a.green as u32) * alpha as u32) + (color_b.green as u32) * (1 - alpha as u32);
        let blue: u32 =
            ((color_a.blue as u32) * alpha as u32) + (color_b.blue as u32) * (1 - alpha as u32);
        Color {
            red: red as u8,
            green: green as u8,
            blue: blue as u8,
        }
    }

    fn color_distance(color_a: Self, color_b: Self) -> u32 {
        let dr: i32 = color_a.red as i32 - color_b.red as i32;
        let dg: i32 = color_a.green as i32 - color_b.green as i32;
        let db: i32 = color_a.blue as i32 - color_b.blue as i32;
        (dr * dr + dg * dg + db * db) as u32
    }

    //ASSUMING PALETTE IS ALWAYS OF LEN 3
    fn closest_mix(self, palette: &[(Self, u8)], max: u32) -> Self {
        let mut result: Color = Color::WHITE;
        let mut distance_result = Color::color_distance(self, result);
        if palette.is_empty() || max == 0 {
            return result;
        }
        let mut nbr_tested = 0;
        for i in 0..palette.len() {
            for j in 0..palette.len() {
                for k in 0..palette.len() {
                    if nbr_tested == max {
                        break;
                    }
                    let mut tmp = Color::color_blending(palette[i].0, palette[i].1, Color::WHITE);
                    tmp = Color::color_blending(palette[j].0, palette[j].1, tmp);
                    tmp = Color::color_blending(palette[k].0, palette[k].1, tmp);
                    if Color::color_distance(self, tmp) < distance_result {
                        distance_result = Color::color_distance(self, tmp);
                        result = tmp;
                    }
                    nbr_tested += 1;
                }
                if nbr_tested == max {
                    break;
                }
            }
            if nbr_tested == max {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Color;

    #[test]
    fn test_constant() {
        let white: Color = Color::WHITE;
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 255);

        let white: Color = Color::RED;
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 0);
        assert_eq!(white.blue, 0);

        let white: Color = Color::GREEN;
        assert_eq!(white.red, 0);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 0);

        let white: Color = Color::BLUE;
        assert_eq!(white.red, 0);
        assert_eq!(white.green, 0);
        assert_eq!(white.blue, 255);
    }

    #[test]
    fn mandatory_test() {
        assert_eq!(Color::RED.closest_mix(&[], 100), Color::WHITE);
        assert_eq!(
            Color::RED.closest_mix(&[(Color::RED, 255)], 0),
            Color::WHITE
        );

        let palette = [(Color::RED, 100), (Color::GREEN, 100), (Color::BLUE, 100)];
        assert_eq!(
            Color::new(254, 23, 102).closest_mix(&palette, 5),
            Color::new(218, 20, 57),
        );
    }
}
