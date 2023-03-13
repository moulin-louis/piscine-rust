pub fn min<'a>( a:&'a i32, b:&'a i32 ) -> &'a i32 {
    if *a < *b {
        a
    }
    else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_min() {
        assert_eq!(min(&5, &4), &4);
        assert_eq!(min(&4, &5), &4);
        assert_eq!(min(&10, &10), &10);
        assert_eq!(min(&-1, &10), &-1);

    }
}
