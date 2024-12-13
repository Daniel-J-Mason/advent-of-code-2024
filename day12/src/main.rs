use common::parse_to_array;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn parse_garden(input: &Vec<String>) -> Vec<Vec<char>> {
    let mut garden: Vec<Vec<char>> = Vec::new();

    for line in input {
        let mut row: Vec<char> = Vec::new();
        for character in line.chars() {
            row.push(character)
        }
        garden.push(row);
    }

    garden
}

fn get_neighbors(garden: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let char = garden[y][x];
    let max_x = garden.first().unwrap().len();
    let max_y = garden.len();

    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for (dx, dy) in directions {
        let x_new: i32 = x as i32 + dx;
        let y_new: i32 = y as i32 + dy;
        if x_new >= 0
            && x_new < max_x as i32
            && y_new >= 0
            && y_new < max_y as i32
            && garden[y_new as usize][x_new as usize] == char
        {
            neighbors.push((x_new as usize, y_new as usize));
        }
    }

    neighbors
}

fn parse_garden_regions(garden: &Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for y in 0..garden.len() {
        for x in 0..garden.first().unwrap().len() {
            let first = (x, y);
            let mut nodes: Vec<(usize, usize)> = vec![first];
            let mut region: Vec<(usize, usize)> = Vec::new();

            if visited.contains(&first) {
                continue;
            }

            while let Some(node) = nodes.pop() {
                if visited.contains(&node) {
                    continue;
                }

                visited.insert(node);
                region.push(node);

                let neighbors = get_neighbors(garden, node.0, node.1);
                for neighbor in neighbors {
                    nodes.push(neighbor);
                }
            }

            regions.insert(first, region);
        }
    }

    regions
}

fn fence_price(
    garden: &Vec<Vec<char>>,
    regions: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> usize {
    let mut fence_price = 0;

    for (_, contents) in regions {
        let mut area = 0;
        let mut perimeter = 0;
        for cell in contents {
            area += 1;
            let neighbor_count = get_neighbors(garden, cell.0, cell.1).len();
            perimeter += 4 - neighbor_count;
        }

        fence_price += area * perimeter
    }

    fence_price
}

fn get_outside_corner_count(region: &Vec<(usize, usize)>) -> usize {
    let mut outside_corner_count = 0;

    for &(x, y) in region {
        // Top left corner
        if (x == 0 || !region.contains(&(x - 1, y)))
            && (y == 0 || !region.contains(&(x, y - 1))) {
            outside_corner_count += 1;
        }

        // Top right corner
        if (y == 0 || !region.contains(&(x, y - 1)))
            && !region.contains(&(x + 1, y)) {
            outside_corner_count += 1;
        }

        // Bottom left corner
        if (x == 0 || !region.contains(&(x - 1, y)))
            && !region.contains(&(x, y + 1)) {
            outside_corner_count += 1;
        }

        // Bottom right corner
        if !region.contains(&(x + 1, y))
            && !region.contains(&(x, y + 1)) {
            outside_corner_count += 1;
        }
    }

    outside_corner_count
}

fn get_inside_corner_count(region: &Vec<(usize, usize)>) -> usize {
    let mut inside_corner_count = 0;

    for &(x, y) in region {
        // Top left corner
        if x > 0
            && y > 0
            && region.contains(&(x - 1, y))
            && region.contains(&(x, y - 1))
            && !region.contains(&(x - 1, y - 1))
        {
            inside_corner_count += 1;
        }

        // Top right corner
        if y > 0
            && region.contains(&(x + 1, y))
            && region.contains(&(x, y - 1))
            && !region.contains(&(x + 1, y - 1))
        {
            inside_corner_count += 1;
        }

        // Bottom left corner
        if x > 0
            && region.contains(&(x - 1, y))
            && region.contains(&(x, y + 1))
            && !region.contains(&(x - 1, y + 1))
        {
            inside_corner_count += 1;
        }

        // Bottom right corner
        if region.contains(&(x + 1, y))
            && region.contains(&(x, y + 1))
            && !region.contains(&(x + 1, y + 1))
        {
            inside_corner_count += 1;
        }
    }

    inside_corner_count
}

fn get_region_corner_count(region: &Vec<(usize, usize)>) -> usize {
    get_inside_corner_count(region) + get_outside_corner_count(region)
}

fn fence_price_by_face(regions: &HashMap<(usize, usize), Vec<(usize, usize)>>) -> usize {
    let mut total = 0;

    for (_, region) in regions {
        total += get_region_corner_count(region) * region.len();
    }

    total
}

fn main() {
    let input = parse_to_array("day12part01.txt").unwrap();
    let garden = parse_garden(&input);
    let garden_regions = parse_garden_regions(&garden);

    let part_one = fence_price(&garden, &garden_regions);
    println!("{}", part_one);

    let part_two = fence_price_by_face(&garden_regions);
    println!("{}", part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let input = vec![
            "RRRRIICCFF".to_string(),
            "RRRRIICCCF".to_string(),
            "VVRRRCCFFF".to_string(),
            "VVRCCCJFFF".to_string(),
            "VVVVCJJCFE".to_string(),
            "VVIVCCJJEE".to_string(),
            "VVIIICJJEE".to_string(),
            "MIIIIIJJEE".to_string(),
            "MIIISIJEEE".to_string(),
            "MMMISSJEEE".to_string(),
        ];

        let garden = parse_garden(&input);
        let garden_regions = parse_garden_regions(&garden);

        assert_eq!(fence_price(&garden, &garden_regions), 1930);
    }

    #[test]
    fn part_two_case_one_test() {
        let input = vec![
            "AAAA".to_string(),
            "BBCD".to_string(),
            "BBCC".to_string(),
            "EEEC".to_string(),
        ];

        let garden = parse_garden(&input);
        let garden_regions = parse_garden_regions(&garden);

        assert_eq!(fence_price_by_face(&garden_regions), 80);
    }

    #[test]
    fn part_two_case_two_test() {
        let input = vec![
            "EEEEE".to_string(),
            "EXXXX".to_string(),
            "EEEEE".to_string(),
            "EXXXX".to_string(),
            "EEEEE".to_string(),
        ];

        let garden = parse_garden(&input);
        let garden_regions = parse_garden_regions(&garden);

        assert_eq!(fence_price_by_face(&garden_regions), 236);
    }

    #[test]
    fn part_two_case_three_test() {
        let input = vec![
            "AAAAAA".to_string(),
            "AAABBA".to_string(),
            "AAABBA".to_string(),
            "ABBAAA".to_string(),
            "ABBAAA".to_string(),
            "AAAAAA".to_string()
        ];

        let garden = parse_garden(&input);
        let garden_regions = parse_garden_regions(&garden);

        assert_eq!(fence_price_by_face(&garden_regions), 368);
    }
}
