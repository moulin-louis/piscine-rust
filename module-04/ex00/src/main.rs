use std::fmt::Debug;

fn print_all_things<I: Debug + IntoIterator>(things: I)
where
    <I as IntoIterator>::Item: Debug,
{
    print!("[");
    for stuff in things {
        print!("{:?} ", stuff);
    }
    println!("]");
}

fn main() {
    print_all_things(0..=5);
    print_all_things("Hello".chars());
    print_all_things(vec![1, 3, 4, 2]);
    print_all_things([1, 2, 5, 4]);
}
