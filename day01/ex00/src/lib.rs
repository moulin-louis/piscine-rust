#[allow(dead_code)]
fn add( a:&i32, b:i32 ) -> i32 {
    *a + b
}

#[allow(dead_code)]
fn add_assign( a:&mut i32, b:i32 ) {
    *a = *a + b;
}

#[cfg(test)]
mod tests {
    use crate::{add, add_assign};

    #[test]
    fn add_test() {
        assert_eq!(add(&1, 1), 2);
        assert_eq!(add(&56, 34), 56 + 34);
        assert_eq!(add(&-2, 6), (-2) + 6);
    }
    #[test]
    fn add_assign_test() {
        let mut a = 1;
        add_assign(&mut a, 1);
        assert_eq!(a, 2);

        a = 56;
        add_assign(&mut a, 34);
        assert_eq!(a, 56 + 34);
        
        a = -2;
        add_assign(&mut a, 6);
        assert_eq!(a, (-2) + 6);
    }
}
