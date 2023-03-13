
pub fn strpcmp( querry: &[u8], pattern: &[u8] ) -> bool {
    if querry.is_empty() {
        return true;
    }
    let mut pos:usize = 0;
    while pos < querry.len() {
        if pattern[pos] == '*' as u8 {
            break ;
        }
        if querry[pos] != pattern[pos] {
            return  false;
        }
        pos += 1;
    }
    pos = querry.len() - 1;
    let mut len_pattern:usize = pattern.len() - 1;
    while len_pattern != 0 {
        if pattern[len_pattern] == '*' as u8 {
            break ;
        }
        if pattern[len_pattern] != querry[pos] {
            return false;
        }
        len_pattern -= 1;
        pos -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::strpcmp;
    #[test]
    fn mandatory_test_1() {
        assert!(strpcmp(b"abc", b"abc"));
    }
    #[test]
    fn mandatory_test_2() {
        assert!(strpcmp(b"abcd", b"ab*"));
    }
    #[test]
    fn mandatory_test_3() {
        assert!(!strpcmp(b"cab", b"ab*"));
    }
    #[test]
    fn mandatory_test_4() {
        assert!(strpcmp(b"dcab", b"*ab"));
    }
    #[test]
    fn mandatory_test_5() {
        assert!(!strpcmp(b"abc", b"*ab"));
    }
    #[test]
    fn mandatory_test_6() {
        assert!(strpcmp(b"ab000cd", b"ab*cd"));
    }
    #[test]
    fn mandatory_test_7() {
        assert!(strpcmp(b"abcd", b"ab*cd"));
    }
    #[test]
    fn mandatory_test_8() {
        assert!(strpcmp(b"", b"****"));
    }  
}
