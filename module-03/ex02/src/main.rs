use std::fmt::{Binary, Debug, Display, Formatter};

struct John {}

impl Display for John {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(x) = f.precision() {
            return match x {
                0 => write!(f, "Dont try to silence me!"),
                _ => f.pad("Hey! I'm John."),
            };
        }
        f.pad("Hey! I'm John.")
    }
}

impl Debug for John {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.pad("John, the man himself. He's handsome AND formidable.")
        } else {
            f.pad("John, the man himself.")
        }
    }
}

impl Binary for John {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad("Bip Boop?")
    }
}

fn main() {
    let john = John {};

    println!("1. {john}");
    println!("2. |{john:<30}|");
    println!("3. |{john:>30}|");
    println!("4. {john:.6}");
    println!("5. {john:.0}");
    println!("6. {john:?}");
    println!("7. {john:#?}");
    println!("8. {john:b}");
}
