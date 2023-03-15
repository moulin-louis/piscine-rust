#[allow(dead_code)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
impl Point {
    fn new( x_i:f32, y_i:f32 ) -> Self {
        Self { x: x_i, y: y_i }
    }
    fn zero( ) -> Self {
        Self::new(0.0, 0.0)
    }
    fn distance( &self, other:&Self ) -> f32 {
       ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
    fn translate( &mut self, dx:f32, dy:f32 ) {
        self.x += dx;
        self.y += dy;
    }
}

#[cfg(test)]
mod test {
    use crate::Point;

    #[test]
    fn test_new() {
        let test:Point = Point::new(42.0, 42.0);
        assert_eq!(42.0, test.x);
        assert_eq!(42.0, test.y);
    }
    #[test]
    fn test_zero() {
        let test:Point = Point::zero();
        assert_eq!(0.0, test.x);
        assert_eq!(0.0, test.y);
    }
    #[test]
    fn test_distance() {
        let test1:Point = Point::new(42.0, 42.0);
        let test2:Point = Point::new(3.0, 14.0);
        assert_eq!(48.010414, test1.distance(&test2));
    }
    #[test]
    fn test_translate() {
        let mut test:Point = Point::new(22.0, 40.0);
        test.translate(20.0, 2.0);
        assert_eq!(test.x, 42.0);
        assert_eq!(test.y, 42.0);
    }
}