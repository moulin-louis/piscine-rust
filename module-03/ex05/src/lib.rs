use std::ops::*;

#[derive(Copy, Clone, Debug)]
struct Vector<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Vector<T> {
    fn new(x: T, y: T) -> Self {
        Vector { x, y }
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> AddAssign for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T> SubAssign for Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vector<T>;
    fn div(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<T> DivAssign<T> for Vector<T>
where
    T: Div<Output = T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl<T> PartialEq for Vector<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[allow(dead_code)]
impl Vector<f32> {
    fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(dead_code)]
impl Vector<f64> {
    fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_a() {
        let v = Vector {
            x: String::from("Hello, World!"),
            y: String::from("Hello, Rust!"),
        };
        let w = v.clone();
        assert_eq!(&v, &w);
    }

    #[test]
    fn test_b() {
        let v = Vector::new("Hello, World!", "Hello, Rust!");
        let a = v;
        let b = v;
        assert_eq!(a, b);
    }

    #[test]
    fn test_c() {
        let mut v = Vector::new(3, 4);
        v += Vector::new(1, 2);
        assert_eq!(v, Vector::new(4, 6));
        v -= Vector::new(1, 2);
        assert_eq!(v, Vector::new(3, 4));
        v = v + Vector::new(1, 2);
        assert_eq!(v, Vector::new(4, 6));
        v = v - Vector::new(1, 2);
        assert_eq!(v, Vector::new(3, 4));
    }

    #[test]
    fn test_d() {
        let mut v = Vector::new(3, 4);
        v = v * 2;
        assert_eq!(v, Vector::new(6, 8));
        v = v / 2;
        assert_eq!(v, Vector::new(3, 4));
        v *= 2;
        assert_eq!(v, Vector::new(6, 8));
        v /= 2;
        assert_eq!(v, Vector::new(3, 4));
    }

    #[test]
    fn test_e() {
        let v: Vector<f32> = Vector::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
        let v: Vector<f64> = Vector::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }
}
