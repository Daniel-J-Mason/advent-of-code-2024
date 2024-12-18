use crate::Direction::{DOWN, LEFT, RIGHT, UP};
use common::parse_to_array;
use std::ops::Add;
use std::process::exit;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Copy, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn translate_direction(direction: Direction) -> Coordinates {
    match direction {
        UP => Coordinates::new(0, -1),
        DOWN => Coordinates::new(0, 1),
        RIGHT => Coordinates::new(1, 0),
        LEFT => Coordinates::new(-1, 0),
    }
}

fn parse_direction(character: char) -> Direction {
    match character {
        '^' => UP,
        'v' => DOWN,
        '<' => LEFT,
        '>' => RIGHT,
        _ => exit(-1),
    }
}

fn parse_grid(grid_input: &Vec<String>) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for line in grid_input {
        let mut row = Vec::new();
        for character in line.chars() {
            row.push(character);
        }
        grid.push(row);
    }

    grid
}

fn parse_movements(movement_input: &Vec<String>) -> Vec<Direction> {
    let mut movements = Vec::new();

    for line in movement_input {
        for character in line.chars() {
            movements.push(parse_direction(character));
        }
    }

    movements
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<char>>, Vec<Direction>) {
    let to_split: Vec<Vec<String>> = input
        .split(|line| line.is_empty())
        .map(|chunk| chunk.to_vec())
        .collect();

    let grid_input = parse_grid(&to_split[0]);
    let movement_input = parse_movements(&to_split[1]);

    (grid_input, movement_input)
}

fn starting_position(grid: &Vec<Vec<char>>) -> Coordinates {
    for (y_index, row) in grid.iter().enumerate() {
        for (x_index, character) in row.iter().enumerate() {
            if *character == '@' {
                return Coordinates::new(x_index as i32, y_index as i32);
            }
        }
    }

    Coordinates::new(0, 0)
}

fn is_movable(grid: &mut Vec<Vec<char>>, coordinates: &Coordinates, direction: &Direction) -> bool {
    let next = *coordinates + translate_direction(*direction);

    match grid[next.y as usize][next.x as usize] {
        '.' => {
            grid[next.y as usize][next.x as usize] =
                grid[coordinates.y as usize][coordinates.x as usize];
            grid[coordinates.y as usize][coordinates.x as usize] = '.';
            true
        }
        'O' => {
            let can_move_next = is_movable(grid, &next, direction);
            if can_move_next {
                grid[next.y as usize][next.x as usize] =
                    grid[coordinates.y as usize][coordinates.x as usize];
                grid[coordinates.y as usize][coordinates.x as usize] = '.';
                true
            } else {
                false
            }
        }
        '#' => false,
        _ => false,
    }
}

fn navigate_grid<'a>(
    grid: &'a mut Vec<Vec<char>>,
    directions: &Vec<Direction>,
) -> &'a mut Vec<Vec<char>> {
    let mut current = starting_position(&grid);

    for direction in directions {
        if is_movable(grid, &current, direction) {
            let next_position = current + translate_direction(*direction);
            current = next_position;
        }
    }

    grid
}

fn gps_score(grid: &Vec<Vec<char>>) -> i64 {
    let mut gps_score = 0;

    for (y_index, row) in grid.iter().enumerate() {
        for (x_index, character) in row.iter().enumerate() {
            if *character == 'O' {
                gps_score += (100 * y_index) + x_index;
            }
        }
    }

    gps_score as i64
}

fn main() {
    let input = parse_to_array("day15part01.txt").unwrap();
    let (mut grid, movements) = parse_input(&input);

    let resulting_grid = navigate_grid(&mut grid, &movements);
    let part_one = gps_score(resulting_grid);
    println!("{}", part_one);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test_small() {
        let input = vec![
            "########".to_string(),
            "#..O.O.#".to_string(),
            "##@.O..#".to_string(),
            "#...O..#".to_string(),
            "#.#.O..#".to_string(),
            "#...O..#".to_string(),
            "#......#".to_string(),
            "########".to_string(),
            "".to_string(),
            "<^^>>>vv<v>>v<<".to_string(),
        ];

        let (mut grid, movements) = parse_input(&input);

        navigate_grid(&mut grid, &movements);
        assert_eq!(gps_score(&grid), 2028);
    }

    #[test]
    fn part_one_test_large() {
        let input = vec![
            "##########".to_string(),
            "#..O..O.O#".to_string(),
            "#......O.#".to_string(),
            "#.OO..O.O#".to_string(),
            "#..O@..O.#".to_string(),
            "#O#..O...#".to_string(),
            "#O..O..O.#".to_string(),
            "#.OO.O.OO#".to_string(),
            "#....O...#".to_string(),
            "##########".to_string(),
            "".to_string(),
            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^".to_string(),
            "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v".to_string(),
            "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<".to_string(),
            "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^".to_string(),
            "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><".to_string(),
            "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^".to_string(),
            ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^".to_string(),
            "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>".to_string(),
            "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>".to_string(),
            "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^".to_string()];

        let (mut grid, movements) = parse_input(&input);

        navigate_grid(&mut grid, &movements);
        assert_eq!(gps_score(&grid), 10092);
    }
}
