fn main() {
    println!("Hey! I'm the other bin target!");
    #[cfg(debug_assertions)]
        let profile: bool = false;
    #[cfg(not(debug_assertions))]
        let profile: bool = true;
    if profile {
        println!("I'm in release mode!");
    }
}