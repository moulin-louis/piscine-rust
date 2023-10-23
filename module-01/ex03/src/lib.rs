pub fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32] {
    if haystack.is_empty() || needle.is_empty() {
        return &[];
    }
    for nbr in needle {
        if !haystack.contains(nbr) {
            return &[];
        }
    }
    let mut result_start: usize = 0;
    let mut result_end: usize = 0;
    for idx in 0..haystack.len() {
        let idx_start: usize = idx;
        let mut idx_end: usize = idx;
        while idx_end != haystack.len() && needle.contains(&haystack[idx_end]) {
            idx_end += 1;
        }
        if idx_end - idx_start > result_end - result_start {
            result_start = idx_start;
            result_end = idx_end;
        }
    }
    if result_end - result_start == 1 {
        return &[];
    }
    &haystack[result_start..result_end]
}

#[cfg(test)]
mod tests {
    use crate::largest_group;

    #[test]
    fn test_lifetimes() {
        let haystack = [1, 2, 3, 2, 1];
        let result;
        {
            let needle = [2, 3];
            result = largest_group(&haystack, &needle);
        }
        assert_eq!(result, &[2, 3, 2]);
    }

    #[test]
    fn mandatory_test() {
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
    }
}
