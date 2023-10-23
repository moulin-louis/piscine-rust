// pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
//     if query.is_empty() || query == pattern {
//         return true;
//     }
//     if pattern.is_empty() || !pattern.contains(&b'*') {
//         return false;
//     }
//     let slice: Vec<&[u8]> = pattern.split(|letter| letter == &b'*').collect();
//     if query.starts_with(slice[0]) && query.ends_with(slice[slice.len() - 1]) {
//         return true;
//     }
//     false
// }

pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
    let mut query_idx = 0;
    let mut pattern_idx = 0;
    let mut last_star_idx = None;

    while query_idx < query.len() && pattern_idx < pattern.len() {
        if pattern[pattern_idx] == b'*' {
            last_star_idx = Some(pattern_idx);
            pattern_idx += 1;
        } else if pattern[pattern_idx] == query[query_idx] || last_star_idx.is_some() {
            query_idx += 1;
            pattern_idx += 1;
        } else if let Some(tmp) = last_star_idx {
            pattern_idx = tmp + 1;
        } else {
            return false;
        }
    }

    while pattern_idx < pattern.len() && pattern[pattern_idx] == b'*' {
        pattern_idx += 1;
    }

    pattern_idx == pattern.len()
}

#[cfg(test)]
mod tests {
    use crate::strpcmp;

    #[test]
    fn test_0() {
        let result = strpcmp(b"abc", b"abc");
        assert!(result);
    }

    #[test]
    fn test_1() {
        let result = strpcmp(b"abcd", b"ab*");
        assert!(result);
    }

    #[test]
    fn test_2() {
        let result = !strpcmp(b"cab", b"ab*");
        assert!(result);
    }

    #[test]
    fn test_3() {
        let result = strpcmp(b"dcab", b"*ab");
        assert!(result);
    }

    #[test]
    fn test_4() {
        let result = strpcmp(b"abcd", b"ab*");
        assert!(result);
    }

    #[test]
    fn test_5() {
        let result = strpcmp(b"ab000cd", b"ab*cd");
        assert!(result);
    }

    #[test]
    fn test_6() {
        let result = strpcmp(b"abcd", b"ab*cd");
        assert!(result);
    }

    #[test]
    fn test_7() {
        let result = strpcmp(b"", b"****");
        assert!(result);
    }
}
