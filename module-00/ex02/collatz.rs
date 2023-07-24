pub fn collatz(start: u32) {
    let mut n = start;
    while n != 1 {
        println!("{}", n);
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };
    }
    println!("{}", n);
}