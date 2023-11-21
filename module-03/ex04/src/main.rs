use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

struct Time {
    hours: u32,
    minutes: u32,
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:02} hours, {:02} minutes", self.hours, self.minutes)
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Time {{ hours: {}, minutes: {} }}",
            self.hours, self.minutes
        )
    }
}

impl FromStr for Time {
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut found_semi: bool = false;
        for char in s.as_bytes() {
            if char == &b':' {
                found_semi = true;
            }
        }
        if !found_semi {
            return Err(TimeParseError::MissingColon);
        }
        if s.len() != 5 {
            return Err(TimeParseError::InvalidLength);
        }
        for char in s.as_bytes() {
            if !char.is_ascii_digit() && char != &b':' {
                return Err(TimeParseError::InvalidNumber);
            }
        }
        let hours: u32 = ((s.as_bytes()[0] - b'0') * 10) as u32 + (s.as_bytes()[1] - b'0') as u32;
        let minutes: u32 = ((s.as_bytes()[3] - b'0') * 10) as u32 + (s.as_bytes()[4] - b'0') as u32;
        Ok(Time { hours, minutes })
    }
}

enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl Debug for TimeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeParseError::MissingColon => write!(f, "Missing colon"),
            TimeParseError::InvalidLength => write!(f, "Invalid length"),
            TimeParseError::InvalidNumber => write!(f, "Invalid number"),
        }
    }
}

impl Display for TimeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeParseError::MissingColon => write!(f, "missing \':\'"),
            TimeParseError::InvalidLength => write!(f, "invalid length"),
            TimeParseError::InvalidNumber => write!(f, "invalid number"),
        }
    }
}

fn main() {
    let a: Time = "12:20".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}
