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

impl<'a, F> Iterator for Groups<'a, F>
where
    F: FnMut(char) -> bool,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let mut idx_start: usize = 0;
        for c in self.s.chars() {
            if (self.f)(c) {
                break;
            }
            idx_start += 1;
        }
        let mut idx_end: usize = 0;
        for c in self.s.chars() {
            if idx_end > idx_start && !(self.f)(c) {
                break;
            }
            idx_end += 1;
        }
        if idx_start == idx_end {
            return None;
        }
        let result = Some(&self.s[idx_start..idx_end]);
        self.s = &self.s[idx_end..self.s.len()];
        result
    }
}

impl<'a, F> DoubleEndedIterator for Groups<'a, F>
where
    F: FnMut(char) -> bool,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut idx_end: usize = self.s.len();
        for c in self.s.chars().rev() {
            if (self.f)(c) {
                break;
            }
            idx_end -= 1;
        }
        let mut idx_start: usize = idx_end - 1;
        for c in self.s.chars().rev() {
            if idx_start < idx_end && !(self.f)(c) {
                break;
            }
            idx_start -= 1;
        }
        if idx_end == idx_start {
            return None;
        }
        let result = Some(&self.s[idx_start + 1..idx_end]);
        self.s = &self.s[0..idx_start];
        result
    }
}

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
        let mut groups = Groups::new("  abc\t def,test", char::is_alphabetic);

        assert_eq!(groups.next(), Some("abc"));
        println!("str = [{:?}]", groups.s);
        assert_eq!(groups.next_back(), Some("test"));
        println!("str = [{:?}]", groups.s);
        assert_eq!(groups.next_back(), Some("def"));
        assert_eq!(groups.next(), None);
    }
}
