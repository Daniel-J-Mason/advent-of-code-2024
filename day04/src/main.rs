use common::parse_to_array;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Index;

lazy_static! {
    static ref XMAS_REGEX: Regex = Regex::new(r"XMAS").unwrap();
    static ref SAMX_REGEX: Regex = Regex::new(r"SAMX").unwrap();

    static ref MS_REGEX: Regex = Regex::new(r"M.S").unwrap();
}

fn get_cols(input: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for i in 0..input.first().unwrap().len() {
        let mut col = String::new();

        for row in input {
            col.push(row.chars().nth(i).unwrap())
        }
        output.push(col);
    }

    output
}

fn get_diagonals(input: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let row_count = input.len();
    let col_count = input.first().unwrap().len();

    for i in 0..=row_count {
        let mut diagonal = String::new();
        let mut x = i;
        let mut y = 0;

        while x < col_count && y < row_count {
            diagonal.push(input.get(y).unwrap().chars().nth(x).unwrap());
            x += 1;
            y += 1;
        }
        if !diagonal.is_empty() {
            output.push(diagonal);
        }
    }

    for i in 1..=col_count {
        let mut diagonal = String::new();
        let mut x = 0;
        let mut y = i;

        while x < col_count && y < row_count {
            diagonal.push(input.get(y).unwrap().chars().nth(x).unwrap());
            x += 1;
            y += 1;
        }

        if !diagonal.is_empty() {
            output.push(diagonal);
        }
    }

    output
}

fn reverse_input(input: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for line in input {
        output.push(line.chars().rev().collect())
    }

    output
}

fn get_reversed_diagonals(input: &Vec<String>) -> Vec<String> {
    get_diagonals(&reverse_input(&input))
}

fn parse_input_to_vectors(input: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    output.extend(get_cols(input));
    output.extend(get_diagonals(input));
    output.extend(get_reversed_diagonals(input));
    output.extend(input.clone()); // already in row format

    output
}

fn find_xmas_count(line: &String) -> usize {
    let mut count = 0;

    count += XMAS_REGEX.find_iter(line).count();
    count += SAMX_REGEX.find_iter(line).count();

    count
}

fn find_total_xmas_count(input: &Vec<String>) -> usize {
    let searchables = parse_input_to_vectors(input);

    let mut count = 0;

    for item in searchables {
        count += find_xmas_count(&item);
    }

    count
}

fn find_x_mas_count(input: &Vec<String>) -> usize {
    let mut count = 0;

    let mut M_S_coordinates: Vec<(usize, usize)> = Vec::new(); // x, y coordinate of an M[wildcard]S capture


    // Stupid hacky sliding match because RUST DOESNT HAVE LOOKAHEAD, WHAT?! and i didnt feel like rewriting everything
    // Should have just made a char array and searched by pure coords
    for row in 0..input.len() {
        let line = input.get(row).unwrap();
        let mut cursor = 0;

        while let Some(capture) = MS_REGEX.captures(&line[cursor..]) {
            if let Some(mat) = capture.get(0) {
                let col_index = cursor + mat.start();
                M_S_coordinates.push((col_index, row));

                cursor = col_index + 1;
            } else {
                break;
            }
        }
    }

    for M_S in &M_S_coordinates {
        if M_S_coordinates.contains(&(M_S.0, M_S.1 + 2)) { // if M_S has a match 2 rows down
            if input.get(M_S.1 + 1).unwrap().chars().nth(M_S.0 + 1).unwrap() == 'A' { // if there's an A in the middle
                count += 1;
            }
        }
    }

    count
}

fn find_total_x_mas_count(input: &Vec<String>) -> usize {
    let mut count = 0;

    count += find_x_mas_count(input); // normal diagonals for M_S
    count += find_x_mas_count(&reverse_input(input)); // S_M case
    count += find_x_mas_count(&get_cols(input)); // vertical M_S case
    count += find_x_mas_count(&reverse_input(&get_cols(input))); // vertical S_M case

    count
}

fn main() {
    let input = parse_to_array("day04part01.txt").unwrap();

    let part_one = find_total_xmas_count(&input);
    println!("{}", part_one);

    let part_two = find_total_x_mas_count(&input);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_diagonals_test() {
        let input = vec![
            "..X...".to_string(),
            ".SAMX.".to_string(),
            ".A..A.".to_string(),
            "XMAS.S".to_string(),
            ".X....".to_string(),
        ];

        let output = vec![
            ".S.S.".to_string(),
            ".A...".to_string(),
            "XMAS".to_string(),
            ".X.".to_string(),
            "..".to_string(),
            ".".to_string(),
            ".AA.".to_string(),
            ".M.".to_string(),
            "XX".to_string(),
            ".".to_string(),
        ];

        assert_eq!(get_diagonals(&input), output)
    }

    #[test]
    fn part_one_test() {
        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];
        assert_eq!(find_total_xmas_count(&input), 18);
    }

    #[test]
    fn part_two_test() {
        let input = vec![
            ".M.S......".to_string(),
            "..A..MSMS.".to_string(),
            ".M.S.MAA..".to_string(),
            "..A.ASMSM.".to_string(),
            ".M.S.M....".to_string(),
            "..........".to_string(),
            "S.S.S.S.S.".to_string(),
            ".A.A.A.A..".to_string(),
            "M.M.M.M.M.".to_string(),
            "..........".to_string(),
        ];
        assert_eq!(find_total_x_mas_count(&input), 9);
    }
}
