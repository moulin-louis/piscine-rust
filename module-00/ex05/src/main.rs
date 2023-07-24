use std::{println, panic};

fn is_leap_year(year: u32) -> bool {
    if year == 0 {
        panic!("Invalid year");
    }
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    if (month < 1) || (month > 12) {
        panic!("Invalid month");
    }
    if month == 2 {
        if is_leap_year(year) {
            return 29;
        }
        return 28;
    }
    if month == 4 || month == 6 || month == 9 || month == 11 {
        return 30;
    }
    31
}

fn convert_month_to_string(month: u32) -> String {
    return match month {
        1 => "January".to_string(),
        2 => "February".to_string(),
        3 => "March".to_string(),
        4 => "April".to_string(),
        5 => "May".to_string(),
        6 => "June".to_string(),
        7 => "July".to_string(),
        8 => "August".to_string(),
        9 => "September".to_string(),
        10 => "October".to_string(),
        11 => "November".to_string(),
        12 => "December".to_string(),
        _ => panic!("Invalid month"),
    }
}

fn main() {
    let mut total_days = 0;  // The total days passed since 1.1.1
    for year in 1..=6 {
        for month in 1..=12 {
            let num_days = num_days_in_month(year, month);
            for day in 1..=num_days {
                total_days += 1;
                // 1.1.1 was a Monday, so 5 days later (total_days % 7 == 5) it's Friday.
                if total_days % 7 == 5 && day == 13 {
                    println!("Friday, {} 13, {}", convert_month_to_string(month), year);
                }
            }
        }
    }
}