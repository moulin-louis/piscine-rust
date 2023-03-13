fn deduplicate( list:&mut Vec<i32> ) {
    for pos in 0..(list.len()) {

        let mut it = pos + 1;

        while it < list.len() {

            if list[it] == list[pos] as i32{
                list.remove(it);
            }

            else {
                it += 1;
            }

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
        let mut v = vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 4, 3];
        deduplicate(&mut v);
        assert_eq!(v, [1, 2, 3, 4]);
    }
    #[test]
    fn bonus_test_1() {
        let mut v = vec![4, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 4, 3];
        deduplicate(&mut v);
        assert_eq!(v, [4, 1, 2, 3]);
    }
}
