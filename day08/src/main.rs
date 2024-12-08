use common::parse_to_array;
use std::collections::{HashMap, HashSet};

fn parse_nodes(input: &Vec<String>) -> (HashMap<char, Vec<(i32, i32)>>, (i32, i32)) {
    let mut nodes: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let max_x = input.first().unwrap().len();
    let max_y = input.len();

    for (line_index, line) in input.iter().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if char != '.' {
                nodes
                    .entry(char)
                    .or_insert(Vec::new())
                    .push((char_index as i32, line_index as i32));
            }
        }
    }

    (nodes, (max_x as i32, max_y as i32))
}

fn calculate_anti_nodes(x1: &i32, y1: &i32, x2: &i32, y2: &i32) -> Vec<(i32, i32)> {
    let x_delta = x1 - x2;
    let y_delta = y1 - y2;

    let new_x1 = x1 + x_delta;
    let new_y1 = y1 + y_delta;

    let new_x2 = x2 - x_delta;
    let new_y2 = y2 - y_delta;

    vec![(new_x1, new_y1), (new_x2, new_y2)]
}

fn is_inbounds(coord: &(i32, i32), bounds: &(i32, i32)) -> bool {
    coord.0 >= 0 && coord.0 < bounds.0 && coord.1 >= 0 && coord.1 < bounds.1
}

fn calculate_extended_nodes(x1: &i32, y1: &i32, x2: &i32, y2: &i32, bounds: &(i32, i32)) -> Vec<(i32, i32)> {
    let x_delta = x1 - x2;
    let y_delta = y1 - y2;

    let mut anti_nodes = vec![(*x1, *y1), (*x2, *y2)];

    let mut new_x = x1 + x_delta;
    let mut new_y = y1 + y_delta;
    while is_inbounds(&(new_x, new_y), bounds) {
        anti_nodes.push((new_x, new_y));
        new_x += x_delta;
        new_y += y_delta;
    }

    let mut new_x = x2 - x_delta;
    let mut new_y = y2 - y_delta;
    while is_inbounds(&(new_x, new_y), bounds) {
        anti_nodes.push((new_x, new_y));
        new_x -= x_delta;
        new_y -= y_delta;
    }

    anti_nodes
}

fn count_anti_nodes(nodes: &HashMap<char, Vec<(i32, i32)>>, bounds: &(i32, i32)) -> i32 {
    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();

    // check all pairs per char by taking first against remaining, second against remaining, etc.
    for (_char, coords) in nodes {
        for i in 0..coords.len() {
            let current = coords[i];
            for j in i + 1..coords.len() {
                let next = coords[j];
                anti_nodes.extend(calculate_anti_nodes(
                    &current.0, &current.1, &next.0, &next.1,
                ))
            }
        }
    }

    anti_nodes
        .iter()
        .filter(|coord| is_inbounds(*coord, bounds))
        .count() as i32
}

fn count_extended_anti_nodes(nodes: &HashMap<char, Vec<(i32, i32)>>, bounds: &(i32, i32)) -> i32 {
    // Unique coords, use set.
    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();

    for (_char, coords) in nodes {
        for i in 0..coords.len() {
            let current = coords[i];
            for j in i + 1..coords.len() {
                let next = coords[j];
                anti_nodes.extend(calculate_extended_nodes(
                    &current.0, &current.1, &next.0, &next.1, &bounds
                ))
            }
        }
    }

    anti_nodes
        .iter()
        .filter(|coord| is_inbounds(*coord, bounds))
        .count() as i32
}

fn main() {
    let input = parse_to_array("day08part01.txt").unwrap();
    let (nodes, bounds) = parse_nodes(&input);

    let part_one = count_anti_nodes(&nodes, &bounds);
    println!("{}", part_one);

    let part_two = count_extended_anti_nodes(&nodes, &bounds);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anti_node_test() {
        assert_eq!(
            calculate_anti_nodes(&8, &9, &9, &10),
            vec![(7, 8), (10, 11)]
        );

        assert_eq!(
            calculate_anti_nodes(&9, &10, &8, &9),
            vec![(10, 11), (7, 8)]
        );

        assert_eq!(
            calculate_anti_nodes(&0, &2, &0, &4),
            vec![(0, 0), (0, 6)]
        );
    }

    #[test]
    fn part_one_test() {
        let input = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        let (nodes, bounds) = parse_nodes(&input);

        assert_eq!(count_anti_nodes(&nodes, &bounds), 14);
    }

    #[test]
    fn part_two_test() {
        let input = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        let (nodes, bounds) = parse_nodes(&input);

        assert_eq!(count_extended_anti_nodes(&nodes, &bounds), 34);
    }
}
