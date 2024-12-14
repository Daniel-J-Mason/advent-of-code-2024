use common::parse_to_array;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BUTTON_A_REGEX: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    static ref BUTTON_B_REGEX: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    static ref PRIZE_REGEX: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
}

fn parse_claw_machines(
    input: &Vec<String>,
) -> Vec<((usize, usize), (usize, usize), (usize, usize))> {
    let mut claw_machines = Vec::new();

    for chunk in input.chunks(4) {
        let button_a_captures = BUTTON_A_REGEX.captures(&chunk[0]).unwrap();
        let button_a = (
            button_a_captures[1].parse::<usize>().unwrap(),
            button_a_captures[2].parse::<usize>().unwrap(),
        );

        let button_b_captures = BUTTON_B_REGEX.captures(&chunk[1]).unwrap();
        let button_b = (
            button_b_captures[1].parse::<usize>().unwrap(),
            button_b_captures[2].parse::<usize>().unwrap(),
        );

        let prize_captures = PRIZE_REGEX.captures(&chunk[2]).unwrap();
        let prize = (
            prize_captures[1].parse::<usize>().unwrap(),
            prize_captures[2].parse::<usize>().unwrap(),
        );

        claw_machines.push((button_a, button_b, prize));
    }

    claw_machines
}

fn min_token_price(a: &(usize, usize), b: &(usize, usize), target: &(usize, usize)) -> usize {
    let mut price = 0;

    let mut a_presses = 0;
    while a.0 * a_presses <= target.0 && a.1 * a_presses <= target.1{
        let test_target = (target.0 - a_presses * a.0, target.1 - a_presses * a.1);

        if (test_target.0 % b.0 == 0) && (test_target.1 % b.1 == 0) {
            let b_presses = test_target.0 / b.0;
            if test_target.1 / b.1 == b_presses {
                price = a_presses * 3 + test_target.0 / b.0;
                break
            }
        }

        a_presses += 1;
    }

    price
}

// [ a.0 b.0 ] [x] = [ target.0 ]
// [ a.1 b.1 ] [y] = [ target.1 ]
fn min_token_price_matrix_solve(a: &(usize, usize), b: &(usize, usize), target: &(usize, usize)) -> usize {

    let a0 = a.0 as i128;
    let a1 = a.1 as i128;
    let b0 = b.0 as i128;
    let b1 = b.1 as i128;
    let t0 = target.0 as i128;
    let t1 = target.1 as i128;



    //invert matrix
    let denom = (a0 * b1) - (b0 * a1);

    // 1/denom * [ b.1 -b.0 ] [ target.0 ] = [x]
    //           [ -a.1 a.0 ] [ target.1 ] = [y]

    let x_num = b1 * t0 - b0 * t1;
    let y_num = a0 * t1 - a1 * t0;

    // Check if int solutions
    if x_num % denom != 0 || y_num % denom != 0 {
        return 0;
    }

    let x = x_num / denom;
    let y = y_num / denom;

    (x * 3 + y) as usize
}

fn fewest_tokens_to_win(input: &Vec<String>) -> usize{
    let mut tokens = 0;

    let claw_machines = parse_claw_machines(input);

    for claw_machine in claw_machines {
        let a = &claw_machine.0;
        let b = &claw_machine.1;
        let target = &claw_machine.2;
        tokens += min_token_price(a, b, target);
    }

    tokens
}

fn fewest_tokens_to_win_10000000000000(input: &Vec<String>) -> usize {
    let mut tokens = 0;

    let claw_machines = parse_claw_machines(input);

    for claw_machine in claw_machines {
        let a = &claw_machine.0;
        let b = &claw_machine.1;
        let target = &claw_machine.2;

        let target = (target.0 + 10000000000000, target.1 + 10000000000000);

        tokens += min_token_price_matrix_solve(a, b, &target);
    }

    tokens
}

fn main() {
    let input = parse_to_array("day13part01.txt").unwrap();

    let part_one = fewest_tokens_to_win(&input);
    println!("{}", part_one);

    // last 93866170395343
    let part_two = fewest_tokens_to_win_10000000000000(&input);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input = vec![
            "Button A: X+94, Y+34".to_string(),
            "Button B: X+22, Y+67".to_string(),
            "Prize: X=8400, Y=5400".to_string(),
            "".to_string(),
        ];

        assert_eq!(parse_claw_machines(&input), vec![((94, 34), (22, 67), (8400, 5400))])
    }

    #[test]
    fn part_one_case_one_test() {
        assert_eq!(min_token_price(&(94, 34), &(22, 67), &(8400, 5400)), 280);
    }

    #[test]
    fn part_one_case_two_test() {
        assert_eq!(min_token_price(&(26, 66), &(67, 21), &(12748, 12176)), 0);
    }

    #[test]
    fn part_one_case_three_test() {
        assert_eq!(min_token_price(&(17, 86), &(84, 37), &(7870, 6450)), 200);
    }

    #[test]
    fn part_one_case_four_test() {
        assert_eq!(min_token_price(&(69, 23), &(27, 71), &(18641, 10279)), 0);
    }

    #[test]
    fn part_two_case_one_test() {
        assert_eq!(min_token_price_matrix_solve(&(94, 34), &(22, 67), &(8400, 5400)), 280);
    }

    #[test]
    fn part_two_case_two_test() {
        assert_eq!(min_token_price_matrix_solve(&(26, 66), &(67, 21), &(12748, 12176)), 0);
    }

    #[test]
    fn part_two_case_three_test() {
        assert_eq!(min_token_price_matrix_solve(&(17, 86), &(84, 37), &(7870, 6450)), 200);
    }

    #[test]
    fn part_two_case_four_test() {
        assert_eq!(min_token_price_matrix_solve(&(69, 23), &(27, 71), &(18641, 10279)), 0);
    }
}
