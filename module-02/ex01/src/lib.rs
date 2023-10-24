#[allow(dead_code)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point{x, y}
    }

    fn zero() -> Self {
        Point::new(0.0, 0.0)
    }

    fn distance(&self, other: &Self) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    #[test]
    fn new_test() {
        let tmp = Point::new(2.5, 4.6);
        assert_eq!(tmp.x, 2.5);
        assert_eq!(tmp.y, 4.6);
    }

    #[test]
    fn zero_test() {
        let tmp = Point::zero();
        assert_eq!(tmp.x, 0.0);
        assert_eq!(tmp.y, 0.0);
    }

    #[test]
    fn distance_test() {
        let point_a = Point::new(4.0, 4.0);
        let point_b = Point::zero();
        assert_eq!(point_a.distance(&point_b), 5.656854);
    }

    #[test]
    fn translate_test() {
        let mut point = Point::zero();
        point.translate(1.0, 1.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 1.0);
    }
}