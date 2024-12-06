use crate::guard_grid::Direction::UP;
use crate::guard_grid::{Coordinate, Direction, Grid, Guard};
use common::parse_to_array;
use std::collections::HashSet;

mod guard_grid;

fn parse_input(input: &Vec<String>) -> Grid {
    let mut contents: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);

    for (line_index, line) in input.iter().enumerate() {
        let mut chars: Vec<char> = Vec::new();

        for (char_index, character) in line.chars().enumerate() {
            if character == '^'{
                start = (char_index as i32, line_index as i32)
            }
            chars.push(character);
        }

        contents.push(chars);
    }

    Grid {
        contents,
        starting_position: Coordinate::new(start.0, start.1)
    }
}

//returns unique steps and whether a loop was detected
fn unique_guard_steps(grid: &Grid) -> (usize, bool) {
    let mut contains_loop = false;

    let mut guard: Guard = Guard {
        direction: UP,
        coordinate: grid.starting_position,
    };

    let mut visited: HashSet<(Coordinate, Direction)> = HashSet::new();
    visited.insert((guard.coordinate, guard.direction));

    let mut unique_coords: HashSet<Coordinate> = HashSet::new();
    unique_coords.insert(guard.coordinate);

    loop {
        let next = guard.coordinate + guard.get_movement();

        if let Some(char) = grid.get_char_at(next) {
            match char {
                '.' | '^' => {
                    guard.take_step();
                    if !visited.insert((guard.coordinate, guard.direction)) {
                        contains_loop = true;
                        break;
                    }
                    unique_coords.insert(guard.coordinate);
                }
                '#' => guard.rotate(),
                _ => break,
            }
        } else {
            break;
        }
    }

    (unique_coords.len(), contains_loop)
}

//permute grid
fn generate_all_possible_grids(grid: &Grid) -> Vec<Grid> {
    let mut grids = Vec::new();

    for (row_index, row) in grid.contents.iter().enumerate() {
        for (col_index, &char) in row.iter().enumerate() {
            if char == '.' {
                let mut permute_contents = grid.contents.clone();

                permute_contents[row_index][col_index] = '#';

                grids.push(
                    Grid {
                        contents: permute_contents,
                        starting_position: grid.starting_position,
                    }
                );
            }
        }
    }

    grids
}

// check all permutations, any with loops are counted
fn get_total_possible_loops(grid: &Grid) -> usize {
    let mut count = 0;

    let grid_permutation = generate_all_possible_grids(grid);

    for grid in grid_permutation {
        if unique_guard_steps(&grid).1 {
            count += 1;
        }
    }

    count
}

fn main() {
    let input = parse_to_array("day06part01.txt").unwrap();
    let grid = parse_input(&input);

    let part_one = unique_guard_steps(&grid);
    println!("{}", part_one.0);

    let part_two = get_total_possible_loops(&grid);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ];

        let grid = parse_input(&input);

        let result = unique_guard_steps(&grid);

        assert_eq!(result.0, 41)
    }

    #[test]
    fn part_two_test() {
        let input = vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ];

        let grid = parse_input(&input);

        let result = get_total_possible_loops(&grid);

        assert_eq!(result, 6)
    }
}
