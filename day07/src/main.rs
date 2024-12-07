use common::parse_to_array;

fn parse_calibration_equations(input: &Vec<String>) -> Vec<Vec<i64>> {
    let mut output: Vec<Vec<i64>> = Vec::new();

    for line in input {
        let mut calibration_equation = Vec::new();
        let slices = line.split(" ");

        for slice in slices {
            let number: i64 = slice.trim_end_matches(':').parse().unwrap();
            calibration_equation.push(number);
        }

        output.push(calibration_equation);
    }

    output
}

fn concat(first: i64, second: i64) -> i64 {
    let first_str = first.to_string();
    let second_str = second.to_string();

    let result_str = format!("{}{}", first_str, second_str);

    result_str.parse::<i64>().unwrap()
}

fn has_operator_match(calibration_equation: &Vec<i64>, with_concat: bool) -> bool {
    let equal_to = &calibration_equation[0];
    let values = &calibration_equation[1..];

    let mut stack: Vec<(usize, i64)> = Vec::new();
    stack.push((0, 0));

    while let Some((index, value)) = stack.pop() {
        if value == *equal_to {
            return true;
        }

        if index < values.len() {
            let next = values[index];
            stack.push((index + 1, value * next));
            stack.push((index + 1, value + next));
            if with_concat {stack.push((index + 1, concat(value, next)))};
        }
    }

    false
}

fn find_total_calibration_sum(calibration_equations: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for calibration_equation in calibration_equations {
        if has_operator_match(calibration_equation, false) {
            result += calibration_equation[0];
        }
    }

    result
}

fn find_total_calibration_sum_with_concatenation(calibration_equations: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for calibration_equation in calibration_equations {
        if has_operator_match(calibration_equation, true) {
            result += calibration_equation[0];
        }
    }

    result
}

fn main() {
    let input = parse_to_array("day07part01.txt").unwrap();
    let calibration_equations = parse_calibration_equations(&input);

    let part_one = find_total_calibration_sum(&calibration_equations);
    println!("{}", part_one);

    let part_two = find_total_calibration_sum_with_concatenation(&calibration_equations);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = vec![
            "190: 10 19".to_string(),
            "3267: 81 40 27".to_string(),
            "83: 17 5".to_string(),
            "156: 15 6".to_string(),
            "7290: 6 8 6 15".to_string(),
            "161011: 16 10 13".to_string(),
            "192: 17 8 14".to_string(),
            "21037: 9 7 18 13".to_string(),
            "292: 11 6 16 20".to_string(),
        ];

        let calibration_equations = parse_calibration_equations(&input);

        assert_eq!(find_total_calibration_sum(&calibration_equations), 3749);
    }
}
