use std::fmt::Debug;

trait FortyTwo {
    fn forty_two(&self) -> Self;
}

impl FortyTwo for u32 {
    fn forty_two(&self) -> Self {
        42
    }
}

impl FortyTwo for String {
    fn forty_two(&self) -> Self {
        "42".to_string()
    }
}

fn print_forty_two<T: Debug + FortyTwo>(x: T) {
    println!("{:?}", x.forty_two());
}

fn main() {
    print_forty_two(42);
    print_forty_two("42".to_string());
}
