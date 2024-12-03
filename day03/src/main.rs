use common::parse_to_text;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MULTIPLICATION_REGEX: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref MULTIPLICATION_REGEX_WITH_DO_DONT: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
}

fn multiplication_sum(input: &String) -> i32 {
    let mut sum = 0;

    for caps in MULTIPLICATION_REGEX.captures_iter(&input) {
        let first: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let second: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        sum += first * second;
    }

    sum
}

fn multiplication_sum_with_do_dont(input: &String) -> i32 {
    let mut enabled: bool = true;
    let mut sum = 0;

    for mat in MULTIPLICATION_REGEX_WITH_DO_DONT.find_iter(input) {
        match mat.as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => if enabled {sum += multiplication_sum(&mat.as_str().to_string())},
        }
    }

    sum
}

fn main() {
    let input = parse_to_text("day03part01.txt").unwrap();

    let part_one = multiplication_sum(&input);
    println!("{}", part_one);

    let part_two = multiplication_sum_with_do_dont(&input);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(
            multiplication_sum(
                &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            ),
            161
        );
    }

    #[test]
    fn multiplication_sum_with_futures_test() {
        assert_eq!(
            multiplication_sum_with_do_dont(
                &"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .to_string()
            ),
            48
        )
    }
}
