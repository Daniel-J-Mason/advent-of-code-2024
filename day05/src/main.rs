use std::collections::HashMap;
use std::hash::Hash;
use std::usize;
use common::parse_to_array;

fn split_input(input: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut delimiter_found = false;
    let mut first: Vec<String> = Vec::new();
    let mut second: Vec<String> = Vec::new();

    for line in input {
        if line == "" {
            delimiter_found = true;
            continue;
        }

        if !delimiter_found {
            first.push(line.clone());
        } else {
            second.push(line.clone());
        }
    }

    (first, second)
}

fn parse_pairs(input: &Vec<String>) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();

    for line in input {
        let parts: Vec<&str> = line.split("|").collect();

        let left: usize = parts[0].parse().unwrap();
        let right: usize = parts[1].parse().unwrap();

        output.push((left, right));
    }

    output
}

fn build_graph(pairs: Vec<(usize, usize)>) -> HashMap<usize, Vec<usize>> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for (left, right) in pairs {
        graph.entry(left).or_insert_with(Vec::new).push(right);
    }

    graph
}

fn parse_updates(update_input: &Vec<String>) -> Vec<Vec<usize>> {
    let mut updates:Vec<Vec<usize>> = Vec::new();

    for line in update_input {
        let mut update: Vec<usize> = Vec::new();

        let list: Vec<&str> = line.split(",").collect();
        for number in list.iter() {
            let number: usize = number.parse().unwrap();
            update.push(number)
        }

        updates.push(update);
    }

    updates
}

fn comparator(graph: &HashMap<usize, Vec<usize>>, left: &usize, right: &usize) -> bool {
    if let Some(neighbors) = graph.get(left) {
        neighbors.contains(right)
    } else {
        false
    }
}

fn update_is_ordered_correctly(graph: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {
    for i in 0..update.len() - 1{
        let current = update.get(i).unwrap();

        for j in i+1.. update.len() {
            let next = update.get(j).unwrap();

            if !comparator(graph, current, next) {
                return false;
            }
        }
    }

    true
}

fn get_middle_element(update: &Vec<usize>) -> usize {
    *update.get(update.len() / 2).unwrap()
}

fn sum_correct_updates(graph: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) -> usize {
    let mut total: usize = 0;

    for update in updates {
        if update_is_ordered_correctly(&graph, update) {
            total += get_middle_element(&update);
        }
    }

    total
}

fn sort_incorrect_update(graph: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> Vec<usize> {
    let mut to_sort = update.clone();

    to_sort.sort_by(|a, b| {
        if comparator(graph, a, b) {
            std::cmp::Ordering::Less
        } else if comparator(graph, b, a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    to_sort
}

fn sum_incorrect_updates(graph: &HashMap<usize, Vec<usize>>, updates: &Vec<Vec<usize>>) -> usize {
    let mut total: usize = 0;

    for update in updates {
        if !update_is_ordered_correctly(&graph, update) {
            let sorted_update = sort_incorrect_update(graph, update);
            total += get_middle_element(&sorted_update);
        }
    }

    total
}

fn main() {
    let input = parse_to_array("day05part01.txt").unwrap();
    let (graph_input, updates_input) = split_input(&input);

    let graph = build_graph(parse_pairs(&graph_input));
    let updates = parse_updates(&updates_input);

    let part_one = sum_correct_updates(&graph, &updates);
    println!("{}", part_one);

    let part_two = sum_incorrect_updates(&graph, &updates);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref GRAPH: HashMap<usize, Vec<usize>> = {
            let input_pairs = vec![
                "47|53".to_string(),
                "97|13".to_string(),
                "97|61".to_string(),
                "97|47".to_string(),
                "75|29".to_string(),
                "61|13".to_string(),
                "75|53".to_string(),
                "29|13".to_string(),
                "97|29".to_string(),
                "53|29".to_string(),
                "61|53".to_string(),
                "97|53".to_string(),
                "61|29".to_string(),
                "47|13".to_string(),
                "75|47".to_string(),
                "97|75".to_string(),
                "47|61".to_string(),
                "75|61".to_string(),
                "47|29".to_string(),
                "75|13".to_string(),
                "53|13".to_string(),
            ];

            let pairs = parse_pairs(&input_pairs);
            build_graph(pairs)
        };
    }

    #[test]
    fn ordering_test() {
        assert_eq!(comparator(&GRAPH, &47, &53), true);
    }

    #[test]
    fn update_order_test() {
        let update = vec![75, 47, 61, 53, 29];
        assert_eq!(update_is_ordered_correctly(&GRAPH, &update), true)
    }

    #[test]
    fn middle_element_test() {
        let update = vec![75, 47, 61, 53, 29];
        assert_eq!(get_middle_element(&update), 61);
    }

    #[test]
    fn part_one_test() {
        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        assert_eq!(sum_correct_updates(&GRAPH, &updates), 143);
    }

    #[test]
    fn part_tow_test() {
        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        assert_eq!(sum_incorrect_updates(&GRAPH, &updates), 123);
    }

}
