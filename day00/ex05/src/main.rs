use std::{assert, assert_eq, assert_ne, panic, print, println};

fn is_leap_year( year:u32 ) -> bool {
    if year % 400 == 0 && year % 100 == 0 {
        return true;
    }
    else if year % 4 == 0 {
        return true;
    }
    false
}

fn num_days_in_month( year:u32, month:u32 ) -> u32 {
    0
}

fn main() {
    for i in 1..
}
