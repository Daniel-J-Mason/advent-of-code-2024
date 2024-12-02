use common::parse_to_array;

fn is_increasing(levels: &Vec<i32>) -> bool {
    let mut sorted_levels: Vec<i32> = levels.clone();
    sorted_levels.sort();

    levels == &sorted_levels
}

fn is_decreasing(levels: &Vec<i32>) -> bool {
    let mut reverse_sorted_levels: Vec<i32> = levels.clone();
    reverse_sorted_levels.sort();
    reverse_sorted_levels.reverse();

    levels == &reverse_sorted_levels
}

fn is_safe(levels: &Vec<i32>) -> bool {
    if !is_increasing(levels) && !is_decreasing(levels) {
        return false;
    }

    for i in 0..levels.len() - 1 {
        let current = levels[i];
        let next = levels[i + 1];

        let difference = current.abs_diff(next);

        if difference > 3 || difference < 1 {
            return false;
        }
    }

    true
}

fn safe_reports_count(reports: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;

    for report in reports {
        if is_safe(report) {
            count += 1;
        }
    }

    count
}

fn generate_report_set(levels: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut set: Vec<Vec<i32>> = Vec::new();

    for i in 0..levels.len() {
        let mut subset = Vec::new();

        for j in 0..i {
            subset.push(levels[j]);
        }

        for j in i+1.. levels.len() {
            subset.push(levels[j]);
        }

        set.push(subset);
    }

    set
}

fn dampened_safe_reports_count(reports: &Vec<Vec<i32>>) -> usize {
    let mut count = 0;

    for report in reports {
        let set = generate_report_set(report);

        for level in set {
            if is_safe(&level) {
                count += 1;
                break;
            }
        }

    }

    count
}

fn parse_reports(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for line in lines {
        let mut numbers: Vec<i32> = Vec::new();

        let split_line = line.split(" ");

        for item in split_line {
            match item.parse::<i32>() {
                Ok(num) => numbers.push(num),
                Err(_) => continue,
            }
        }

        result.push(numbers);
    }

    result
}

fn main() {
    let input = parse_to_array("day02part01.txt").unwrap();
    let reports = parse_reports(&input);

    let part_one = safe_reports_count(&reports);
    println!("{}", part_one);

    let part_two = dampened_safe_reports_count(&reports);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_count_test() {
        let reports = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];

        let input = parse_reports(&reports);
        assert_eq!(safe_reports_count(&input), 2);
    }

    #[test]
    fn dampened_safe_count_test() {
        let reports = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];

        let input = parse_reports(&reports);
        assert_eq!(dampened_safe_reports_count(&input), 4);
    }
}
