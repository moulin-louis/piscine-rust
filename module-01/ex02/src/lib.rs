fn handle_other_case<'a>(color: &[u8; 3]) -> &'a str {
    if color[0] < 31 && color[1] < 31 && color[2] < 31 {
        return "almost black";
    }
    if color[0] > 128 && color[1] < 128 && color[2] < 128 {
        return "redish";
    }
    if color[0] < 128 && color[1] > 128 && color[2] < 128 {
        return "greenish";
    }
    if color[0] < 128 && color[1] < 128 && color[2] > 128 {
        return "blueish";
    }
    "unknown"
}

pub fn color_name<'a>(color: &[u8; 3]) -> &'a str {
    match color {
        [0, 0, 0] => "pure black",
        [255, 255, 255] => "pure white",
        [255, 0, 0] => "pure red",
        [0, 255, 0] => "pure green",
        [0, 0, 255] => "pure blue",
        [128, 128, 128] => "pure grey",
        _ => return handle_other_case(color),
    }
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
    fn test_rest() {
        assert_eq!(color_name(&[0, 0, 0]), "pure black");
        assert_eq!(color_name(&[255, 255, 255]), "pure white");
        assert_eq!(color_name(&[255, 0, 0]), "pure red");
        assert_eq!(color_name(&[0, 255, 0]), "pure green");
        assert_eq!(color_name(&[0, 0, 255]), "pure blue");
        assert_eq!(color_name(&[128, 128, 128]), "pure grey");
        assert_eq!(color_name(&[12, 15, 8]), "almost black");
        assert_eq!(color_name(&[167, 112, 45]), "redish");
        assert_eq!(color_name(&[0, 253, 0]), "greenish");
        assert_eq!(color_name(&[127, 127, 129]), "blueish");
        assert_eq!(color_name(&[245, 245, 245]), "unknown");
    }
}
