use common::parse_to_array;

fn parse_input_to_sorted_lists(input: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for pair in input {
        let mut pair_iter = pair.split("   ");

        let num1: u32 = pair_iter.next()
            .unwrap()
            .parse()
            .unwrap();
        let num2: u32 = pair_iter.next()
            .unwrap()
            .parse()
            .unwrap();

        vec1.push(num1);
        vec2.push(num2);
    }

    vec1.sort();
    vec2.sort();

    (vec1, vec2)
}

fn find_total_distance(first: &Vec<u32>, second: &Vec<u32>) -> u32 {
    let mut sum = 0;

    for i in 0..first.len() {
        sum += first.get(i).unwrap().abs_diff(*second.get(i).unwrap());
    }

    sum
}

fn times_found_in(number: &u32, list_of_numbers: &Vec<u32>) -> usize {
    list_of_numbers.iter().filter(|&x| x == number).count()
}

fn find_similarity_score(first: &Vec<u32>, second: &Vec<u32>) -> usize {
    let mut score = 0;

    for i in 0..first.len() {
        let number = first.get(i).unwrap();
        score += *number as usize * times_found_in(number, &second)
    }

    score
}

fn main() {
    let input = parse_to_array("day01part01.txt").unwrap();
    let vecs = parse_input_to_sorted_lists(&input);

    let part_one = find_total_distance(&vecs.0, &vecs.1);
    println!("{}", part_one);

    let part_two = find_similarity_score(&vecs.0, &vecs.1);
    println!("{}", part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_distance_test() {
        let input_vec = vec!["3   4".to_string(),
                             "4   3".to_string(),
                             "2   5".to_string(),
                             "1   3".to_string(),
                             "3   9".to_string(),
                             "3   3".to_string()];
        let vecs = parse_input_to_sorted_lists(&input_vec);
        assert_eq!(find_total_distance(&vecs.0, &vecs.1), 11);
    }

    #[test]
    fn similarity_score_test() {
        let input_vec = vec!["3   4".to_string(),
                             "4   3".to_string(),
                             "2   5".to_string(),
                             "1   3".to_string(),
                             "3   9".to_string(),
                             "3   3".to_string()];
        let vecs = parse_input_to_sorted_lists(&input_vec);
        assert_eq!(find_similarity_score(&vecs.0, &vecs.1), 31);
    }
}