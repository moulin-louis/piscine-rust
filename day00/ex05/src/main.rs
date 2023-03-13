use std::panic;

fn is_leap_year( year:u32 ) -> bool {
	if year == 0 || year > 2024 { panic!(); }
	if year % 400 == 0 && year % 100 == 0 { return true; }
	else if year % 100 == 0 && year % 400 != 0 { return false; }
	else if year % 4 == 0 { return true; }
	false
}

fn num_days_in_month( year:u32, month:u32 ) -> u32 {
	if month > 12 {
		panic!();
	}
	if is_leap_year(year) && month == 2 {
		return 29;
	}
	if month == 4 || month == 6 || month == 9 || month == 11 {
		return 30;
	}
	else if month != 2 {
		return 31;
	}
	28
}

fn return_month( month:u32 ) -> String {
	if month == 1 { return String::from("January"); }
	if month == 2 {	return String::from("February");	}
	if month == 3 { return String::from("March"); }
	if month == 4 {	return String::from("April"); }
	if month == 5 {	return String::from("May"); }
	if month == 6 {	return String::from("June"); }
	if month == 7 {	return String::from("July");	}
	if month == 8 {	return String::from("August"); }
	if month == 9 {	return String::from("September"); }
	if month == 10 { return String::from("October");	}
	if month == 11 { return String::from("November"); }
	String::from("December")
}

fn main() {
	let mut day_of_week:u32 = 0;
	for year in 1..2024 {
		for month in 1..13 {
			for day in 1..(num_days_in_month(year, month) + 1) {
				if day == 13 && day_of_week == 4 {
					println!("Friday, {} 13, {}", return_month(month), year);
				}
				day_of_week += 1;
				if day_of_week == 7 {
					day_of_week = 0;
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{is_leap_year, num_days_in_month};

	#[test]
	fn check_leap_year_1600() {
		assert!(is_leap_year(1600));
	}
	#[test]
	fn check_non_leap_year_1500() {
		assert!(!is_leap_year(1500));
	}
	#[test]
	fn check_non_leap_year_2003() {
		assert!(!is_leap_year(2003));
	}
	#[test]
	fn check_leap_year_2004() {
		assert!(is_leap_year(2004));
	}
	#[test]
	fn check_day_month_feb_leap() {
		assert_eq!(num_days_in_month(1600, 2), 29);
	}
	#[test]
	fn check_day_month_feb_non_leap() {
		assert_eq!(num_days_in_month(1601, 2), 28);
	}
	#[test]
	fn check_day_month_december_non_leap() {
		assert_eq!(num_days_in_month(1601, 12), 31);
	}
	#[test]
	fn check_day_month_december_leap() {
		assert_eq!(num_days_in_month(1600, 12), 31);
	}
	#[test]
	fn check_day_month_june_non_leap() {
		assert_eq!(num_days_in_month(1601, 06), 30);
	}
	#[test]
	fn check_day_month_june_leap() {
		assert_eq!(num_days_in_month(1600, 06), 30);
	}
	#[test]
	#[should_panic]
	fn check_panic_leap_year() {
		is_leap_year(0);
	}
	#[test]
	#[should_panic]
	fn check_panic_invalid_month() {
		num_days_in_month(24, 24);
	}
}