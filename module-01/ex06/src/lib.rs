fn atoi(arr: &[u8]) -> u32{
    let mut result: u32 = 0;
    for idx in 0..arr.len() {
        result = result * 10 + (arr[idx] - '0' as u8) as u32;
    }
    result
}

fn size_nbr(nbr: u32) -> u32 {
    let mut nbr: u32 = nbr;
    let mut result: u32 = 0;
    if nbr == 0 {
        return 1;
    }
    while nbr > 1 {
        nbr /= 10;
        result += 1;
    }
    if nbr == 1 {
        result += 1;
    }
    result
}

fn itoa(nbr: u32) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut nbr: u32 = nbr;
    let mut len = size_nbr(nbr);
    while len != 0 {
        result.push((nbr % 10) as u8 + '0' as u8);
        nbr /= 10;
        len -= 1;
    }
    result.reverse();
    result
}

pub fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    if a.is_empty() || b.is_empty() {
        panic!("A or B is empty!");
    }
    for letter in a {
        if !(*letter as char).is_ascii_digit() {
            panic!("Find illegal char in a");
        }
    }
    for letter in b {
        if !(*letter as char).is_numeric() {
            panic!("Find illegal char in b");
        }
    }

    let mut a_start = 0;
    while a[a_start] == '0' as u8 {
        a_start += 1;
    }
    let mut b_start = 0;
    while b[b_start] == '0' as u8 {
        b_start += 1;
    }
    let a_nbr = atoi(&a[a_start..a.len()]);
    let b_nbr = atoi(&b[b_start..b.len()]);
    let result_nbr = a_nbr + b_nbr;
    itoa(result_nbr)
}

#[cfg(test)]
mod tests {
    use crate::big_add;

    #[test]
    fn mandatory_test() {
        assert_eq!(big_add(b"2", b"4"), b"6");
        assert_eq!(big_add(b"0010", b"0200"), b"210");
    }
}
