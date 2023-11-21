use ftkit::random_number;

fn choose<T>(values: &[T]) -> &T {
    if values.len() == 0 {
        panic!("Cannot choose from an empty list");
    }
    let random_index = random_number(0..(values.len() as i32));
    &values[random_index as usize]
}
fn main() {
    let values = vec![1, 2, 3, 4, 5];
    println!("first random value: {}", choose(&values));
    println!("second random value: {}", choose(&values));
    //test empty list
    let empty_values: Vec<i32> = vec![];
    println!("first random value: {}", choose(&empty_values));
}
