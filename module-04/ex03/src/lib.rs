struct Fibs {
    curr: u32,
    next: u32,
}

impl Iterator for Fibs {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next.checked_add(current)?;
        self.next += current;
        Some(current)
    }
}

#[allow(dead_code)]
impl Fibs {
    fn new(curr: u32, next: u32) -> Self {
        Self { curr, next }
    }
}

#[allow(dead_code)]
fn even_fibs_bellow_1000() -> u32 {
    Fibs::new(0, 1)
        .take_while(|x| x < &1000)
        .filter(|x| x % 2 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mandatory_test() {
        let mut count = 0;
        for fib in Fibs::new(0, 1) {
            if fib >= 1000 {
                break;
            }
            if fib % 2 == 0 {
                count += fib;
            }
        }

        assert_eq!(count, 798);
        assert_eq!(even_fibs_bellow_1000(), 798);
    }
}
