use std::collections::{HashSet, VecDeque};
use common::parse_to_array;

fn parse_mountain(input: &Vec<String>) -> Vec<Vec<usize>> {
    let mut mountain: Vec<Vec<usize>> = Vec::new();

    for line in input {
        let mut vec: Vec<usize> = Vec::new();
        for char in line.chars() {
            let number: usize = char.to_digit(10).unwrap_or(11) as usize;
            vec.push(number);
        }
        mountain.push(vec);
    }

    mountain
}

fn get_valid_neighbors(mountain: &Vec<Vec<usize>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let value = mountain[y][x];

    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dx, dy) in directions {
        let x_new = x as i32 + dx;
        let y_new = y as i32 + dy;
        if x_new >= 0
            && x_new < mountain[0].len() as i32
            && y_new >= 0
            && y_new < mountain.len() as i32
        {
            if mountain[y_new as usize][x_new as usize] == value + 1 {
                neighbors.push((x_new as usize, y_new as usize));
            }
        }
    }

    neighbors
}

fn find_trailhead_scores(mountain: &Vec<Vec<usize>>, distinct_paths: bool) -> usize {
    let mut trail_starts = Vec::new();

    let mut total_trail_scores = 0;

    for (y_index, row) in mountain.iter().enumerate() {
        for x in 0..row.len() {
            if mountain[y_index][x] == 0 {
                trail_starts.push((x, y_index));
            }
        }
    }

    for trail_start in trail_starts {
        total_trail_scores += bfs(mountain, trail_start.0, trail_start.1, distinct_paths);
    }

    total_trail_scores
}

fn bfs(mountain: &Vec<Vec<usize>>, x: usize, y: usize, distinct_paths: bool) -> usize {
    let mut peaks_found = 0;

    let mut nodes: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    nodes.push_back((x, y));

    while !nodes.is_empty() {
        let node = nodes.pop_front().unwrap();

        if !distinct_paths {
            if visited.contains(&node) {
                continue
            }

            visited.insert(node);
        }

        if mountain[node.1][node.0] == 9 {
            peaks_found += 1;
            continue;
        }

        let neighbors = get_valid_neighbors(&mountain, node.0, node.1);

        for neighbor in neighbors {
            nodes.push_back(neighbor);
        }
    }

    peaks_found
}

fn main() {
    let input = parse_to_array("day10part01.txt").unwrap();
    let mountain = parse_mountain(&input);

    let part_one = find_trailhead_scores(&mountain, false);
    println!("{}", part_one);

    let part_two = find_trailhead_scores(&mountain, true);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dual_peak_test() {
        let input = vec![
            "...0...".to_string(),
            "...1...".to_string(),
            "...2...".to_string(),
            "6543456".to_string(),
            "7.....7".to_string(),
            "8.....8".to_string(),
            "9.....9".to_string(),
        ];

        let mountain = parse_mountain(&input);

        assert_eq!(find_trailhead_scores(&mountain, false), 2);
    }

    #[test]
    fn four_peak_test() {
        let input = vec![
            "..90..9".to_string(),
            "...1.98".to_string(),
            "...2..7".to_string(),
            "6543456".to_string(),
            "765.987".to_string(),
            "876....".to_string(),
            "987....".to_string(),
        ];

        let mountain = parse_mountain(&input);

        assert_eq!(find_trailhead_scores(&mountain, false), 4);
    }

    #[test]
    fn part_one_test() {
        let input = vec![
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ];

        let mountain = parse_mountain(&input);

        assert_eq!(find_trailhead_scores(&mountain, false), 36);
    }

    #[test]
    fn part_two_test() {
        let input = vec![
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ];

        let mountain = parse_mountain(&input);

        assert_eq!(find_trailhead_scores(&mountain, true), 81);
    }
}
