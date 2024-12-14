use common::parse_to_array;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

lazy_static! {
    static ref ROBOT_REGEX: Regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
}

fn parse_robots(inputs: &Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    let mut robots = Vec::new();

    for input in inputs {
        let robot_captures = ROBOT_REGEX.captures(input).unwrap();
        let robot = (
            (
                robot_captures[1].parse::<i32>().unwrap(),
                robot_captures[2].parse::<i32>().unwrap(),
            ),
            (
                robot_captures[3].parse::<i32>().unwrap(),
                robot_captures[4].parse::<i32>().unwrap(),
            ),
        );

        robots.push(robot);
    }

    robots
}

fn move_robots(robots: &mut Vec<((i32, i32), (i32, i32))>, size: (i32, i32)) {
    for robot in robots {
        let p = &robot.0;
        let v = &robot.1;

        let mut new_p0 = p.0 + v.0;
        let mut new_p1 = p.1 + v.1;

        if new_p0 < 0 {
            new_p0 = (new_p0 % size.0 + size.0) % size.0;
        } else if new_p0 >= size.0 {
            new_p0 = new_p0 % size.0;
        }

        if new_p1 < 0 {
            new_p1 = (new_p1 % size.1 + size.1) % size.1;
        } else if new_p1 >= size.1 {
            new_p1 = new_p1 % size.1;
        }

        *robot = ((new_p0, new_p1), *v);
    }
}

fn print_robots(robots: &Vec<((i32, i32), (i32, i32))>, size: (i32, i32)) {
    print!("\x1B[2J\x1B[1;1H");
    let mut grid = vec![vec![0; size.0 as usize]; size.1 as usize];

    for robot in robots {
        let x = robot.0 .0;
        let y = robot.0 .1;

        grid[y as usize][x as usize] += 1;
    }

    for row in grid {
        for cell in row {
            if cell == 0 {
                print!(".")
            } else {
                print!("#")
            }
        }
        println!();
    }
}

fn robot_safety_factor(robots: &mut Vec<((i32, i32), (i32, i32))>, size: (i32, i32)) -> i32 {
    for _ in 0..100 {
        move_robots(robots, size);
    }

    let mid_x = size.0 / 2;
    let mid_y = size.1 / 2;

    println!("{} {}", mid_x, mid_y);

    let mut first_quadrant = 0;
    let mut second_quadrant = 0;
    let mut third_quadrant = 0;
    let mut fourth_quadrant = 0;

    for robot in robots {
        let x = &robot.0 .0;
        let y = &robot.0 .1;

        if x > &mid_x && y > &mid_y {
            fourth_quadrant += 1;
        } else if x < &mid_x && y > &mid_y {
            third_quadrant += 1;
        } else if x > &mid_x && y < &mid_y {
            second_quadrant += 1;
        } else if x < &mid_x && y < &mid_y {
            first_quadrant += 1;
        }
    }

    first_quadrant * second_quadrant * third_quadrant * fourth_quadrant
}

fn find_first_repeat(robots: &mut Vec<((i32, i32), (i32, i32))>, size: (i32, i32)) -> i32 {
    let mut cache = HashMap::new();
    let mut i = 0;

    loop {
        if let Some(prev_iteration) = cache.get(robots) {
            return i - *prev_iteration;
        }

        cache.insert(robots.clone(), i);
        move_robots(robots, size);

        i += 1;
    }
}

fn main() {
    let input = parse_to_array("day14part01.txt").unwrap();
    let robots = &mut parse_robots(&input);

    let part_one = robot_safety_factor(robots, (101, 103));
    println!("{}", part_one);

    let robots = &mut parse_robots(&input);

    for _ in 0..18 {
        move_robots(robots, (101, 103));
    }

    // Found tree-like pattern at iteration 18, repeats every 101 iterations
    for i in 18..10403 {
        print_robots(robots, (101, 103));
        println!("Iteration #{}", i);
        for _ in 0..101 {
            move_robots(robots, (101, 103));
        }

        sleep(Duration::from_millis(1000));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let robot = "p=0,4 v=3,-3";
        let robot = parse_robots(&vec![robot.to_string()]);
        assert_eq!(robot, vec![((0, 4), (3, -3))])
    }

    #[test]
    fn test_visualization_one() {
        let input = vec![
            "p=0,4 v=3,-3".to_string(),
            "p=6,3 v=-1,-3".to_string(),
            "p=10,3 v=-1,2".to_string(),
            "p=2,0 v=2,-1".to_string(),
            "p=0,0 v=1,3".to_string(),
            "p=3,0 v=-2,-2".to_string(),
            "p=7,6 v=-1,-3".to_string(),
            "p=3,0 v=-1,-2".to_string(),
            "p=9,3 v=2,3".to_string(),
            "p=7,3 v=-1,2".to_string(),
            "p=2,4 v=2,-3".to_string(),
            "p=9,5 v=-3,-3".to_string(),
        ];

        let robots = parse_robots(&input);

        print_robots(&robots, (11, 7));
    }

    #[test]
    fn test_visualization_two() {
        let input = vec!["p=2,4 v=2,-3".to_string()];

        let robots = &mut parse_robots(&input);

        print_robots(&robots, (11, 7));
        println!();
        move_robots(robots, (11, 7));
        print_robots(&robots, (11, 7));
        println!();
        move_robots(robots, (11, 7));
        print_robots(&robots, (11, 7));
    }

    #[test]
    fn part_one_test() {
        let input = vec![
            "p=0,4 v=3,-3".to_string(),
            "p=6,3 v=-1,-3".to_string(),
            "p=10,3 v=-1,2".to_string(),
            "p=2,0 v=2,-1".to_string(),
            "p=0,0 v=1,3".to_string(),
            "p=3,0 v=-2,-2".to_string(),
            "p=7,6 v=-1,-3".to_string(),
            "p=3,0 v=-1,-2".to_string(),
            "p=9,3 v=2,3".to_string(),
            "p=7,3 v=-1,2".to_string(),
            "p=2,4 v=2,-3".to_string(),
            "p=9,5 v=-3,-3".to_string(),
        ];

        let robots = &mut parse_robots(&input);

        let result = robot_safety_factor(robots, (11, 7));

        print_robots(&robots, (11, 7));

        assert_eq!(result, 12)
    }
}
