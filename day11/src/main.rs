use std::collections::HashMap;
use std::time::Instant;
use common::parse_to_text;

fn parse_stones(input: &String) -> Vec<i64> {
    let mut stones: Vec<i64> = Vec::new();

    for stone in input.split(" ") {
        if let Ok(value) = stone.parse::<i64>() {
            stones.push(value);
        }
    }

    stones
}

fn even_length(value: &i64) -> bool {
    let digits = ((*value as f64).log10().floor() as u32) + 1;
    digits % 2 == 0
}

fn split(value: &i64) -> (i64, i64) {
    let digits = ((*value as f64).log10().floor() as u32) + 1;
    let half = digits / 2;

    let divisor = 10_i64.pow(half);
    let first_half = value / divisor;
    let second_half = value % divisor;

    (first_half, second_half)
}

fn blink(stones: &mut Vec<i64>) {
    let mut new_stones: Vec<i64> = Vec::with_capacity(stones.len() * 2);
    for (_, stone_value) in stones.iter().enumerate() {
        if *stone_value == 0 {
            new_stones.push(1);
        } else if even_length(stone_value) {
            let (first_half, second_half) = split(stone_value);
            new_stones.push(first_half);
            new_stones.push(second_half);
        } else {
            new_stones.push(stone_value * 2024);
        }
    }

    stones.clear();
    stones.extend(new_stones);
}

fn stones_after_n_blinks(stones: &mut Vec<i64>, count: usize) -> Vec<i64> {
    for i in 0..count {
        blink(stones);
        println!("{}, length: {}", i, stones.len());
    }

    stones.clone()
}

fn stone_count_after_n_blinks(stones: &mut Vec<i64>, blinks: usize, memo: &mut HashMap<(i64, usize), usize>) -> usize {
    if blinks == 0 {
        return stones.len();
    }

    let mut count = 0;

    for &stone in stones.iter() {
        if let Some(&cached_result) = memo.get(&(stone, blinks)) {
            count += cached_result;
        } else {
            let mut single_stone = vec![stone];
            blink(&mut single_stone);

            let recursive_result =
                single_stone.iter()
                    .map(|s| stone_count_after_n_blinks(&mut vec![*s], blinks - 1, memo))
                    .sum();

            memo.insert((stone, blinks), recursive_result);
            count += recursive_result;
        }
    }

    count
}

fn main() {
    let input = parse_to_text("day11part01.txt").unwrap();
    let stones = &mut parse_stones(&input);

    let start_part_one = Instant::now();
    let part_one = stones_after_n_blinks(stones, 25).len();
    let duration_part_one = start_part_one.elapsed();
    println!("{}", part_one);
    println!("Part 1 completed in: {:?}", duration_part_one);

    let stones = &mut parse_stones(&input);
    let memo = &mut HashMap::new();
    let start_part_two = Instant::now();
    let part_two = stone_count_after_n_blinks(stones, 75, memo);
    let duration_part_two = start_part_two.elapsed();
    println!("{}", part_two);
    println!("Part 2 completed in: {:?}", duration_part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        assert_eq!(split(&100), (10, 0));
        assert_eq!(split(&1000), (10, 0));
        assert_eq!(split(&1234), (12, 34));
        assert_eq!(split(&123456), (123, 456));
    }

    #[test]
    fn blink_test() {
        let input = &mut vec![125, 17];
        blink(input);

        assert_eq!(input.clone(), vec![253000, 1, 7])
    }

    #[test]
    fn part_one_test() {
        let input = &mut vec![125, 17];
        assert_eq!(stones_after_n_blinks(input, 6), vec![2097446912,14168,4048,2,0,2,4,40,48,2024,40,48,80,96,2,8,6,7,6,0,3,2]);

        let input = &mut vec![125, 17];
        assert_eq!(stones_after_n_blinks(input, 6).len(), 22);

        let input = &mut vec![125, 17];
        assert_eq!(stones_after_n_blinks(input, 25).len(), 55312);
    }

    #[test]
    fn part_two_test() {
        let input = &mut vec![125, 17];
        assert_eq!(stone_count_after_n_blinks(input, 6, &mut HashMap::new()), 22);

        let input = &mut vec![125, 17];
        assert_eq!(stone_count_after_n_blinks(input, 25, &mut HashMap::new()), 55312);
    }
}
