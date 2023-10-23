pub fn deduplicate(list: &mut Vec<i32>) {
    if list.is_empty() {
        return;
    }
    loop {
        let mut ended = true;
        let len: usize = list.len();
        for idx in 0..len {
            let mut breaked: bool = false;
            for idx2 in (idx + 1)..len {
                if list[idx] == list[idx2] {
                    list.remove(idx2);
                    ended = false;
                    breaked = true;
                    break;
                }
            }
            if breaked {
                break;
            }
        }
        if ended {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::deduplicate;

    #[test]
    fn mandatory_test() {
        let mut v = vec![1, 2, 2, 3, 2, 4, 3];
        deduplicate(&mut v);
        assert_eq!(v, [1, 2, 3, 4]);
    }

    #[test]
    fn bonus_test() {
        let mut v = vec![1, 1, 1, 1, 1, 1, 2, 3, 4];
        deduplicate(&mut v);
        assert_eq!(v, [1, 2, 3, 4]);
        let mut v = vec![1, 1, 1, 1, 1, 1, 2, 3, 4, 1];
        deduplicate(&mut v);
        assert_eq!(v, [1, 2, 3, 4]);
        let mut v = vec![1, 1, 1, 1, 1, 1, 2, 3, 4, 4, 4, 4, 4];
        deduplicate(&mut v);
        assert_eq!(v, [1, 2, 3, 4]);
    }
}
