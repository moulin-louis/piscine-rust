struct Groups<'a, F> {
    s: &'a str,
    f: F,
}

#[allow(dead_code)]
impl<'a, F> Groups<'a, F> {
    pub fn new(s: &'a str, f: F) -> Self
    where
        F: FnMut(char) -> bool,
    {
        Groups { s, f }
    }
}

impl Iterator for Groups<F> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mandatory_test_1() {
        let mut groups = Groups::new("  hello,\tworld ", char::is_alphabetic);

        assert_eq!(groups.next(), Some("hello"));
        assert_eq!(groups.next(), Some("world"));
        assert_eq!(groups.next(), None);
    }

    #[test]
    fn mandatory_test_2() {
        let mut groups = Groups::new("  abc\t def,test");

        assert_eq!(groups.next(), Some("abc"));
        assert_eq!(groups.next_back(), Some("test"));
        assert_eq!(groups.next_back(), Some("def"));
        assert_eq!(groups.next(), None);
    }
}
