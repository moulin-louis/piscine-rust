use std::fmt::Debug;
use std::str::FromStr;

pub fn sum_of_odds(s: &str) -> u32 {
    s.split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .filter(|x| x % 2 != 0)
        .sum()
}

pub fn create_pairs<T: FromStr>(s: &str) -> Vec<(&str, T)>
where
    <T as FromStr>::Err: Debug,
{
    s.split('\n')
        .filter(|x| x.split_once(':').is_some())
        .map(|line| {
            (
                line.split_once(':').unwrap().0.trim().trim_end(),
                line.split_once(':').unwrap().1.trim().parse::<T>().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mandatory_test_1() {
        let sum = sum_of_odds("hey 20 test 3\t9 4 5, 1 hello");
        assert_eq!(sum, 13);
    }

    #[test]
    fn mandatory_test_2() {
        let input = "\
            first: 1
            second 2
            \t third   : 3
            fourth
            fifth  : 43\t
            \tsixth
        ";

        let v: Vec<(&str, u32)> = create_pairs(input);
        assert_eq!(v, [("first", 1), ("third", 3), ("fifth", 43),]);
    }
}
