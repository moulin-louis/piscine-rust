#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<std::cmp::Ordering> {
        if self.x == other.x && self.y == other.y {
            Some(std::cmp::Ordering::Equal)
        } else if self.x <= other.x && self.y <= other.y {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

pub fn min<T: PartialOrd>(val1: T, val2: T) -> T {
    if val1 < val2 {
        val1
    } else {
        val2
    }
}

#[cfg(test)]
mod tests {
    use crate::min;
    use crate::Point;

    #[test]
    fn it_works() {
        assert_eq!(min(12i32, -14i32), -14);
        assert_eq!(min(12f32, 14f32), 12f32);
        assert_eq!(min("abc", "def"), "abc");
        assert_eq!(min(String::from("abc"), String::from("def")), "abc");
        assert_eq!(
            min(Point { x: 4.0, y: 4.0 }, Point { x: 3.0, y: 3.0 }),
            Point { x: 3.0, y: 3.0 }
        );
    }
}
