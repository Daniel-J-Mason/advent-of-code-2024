use itertools::Itertools;
use std::ops::Index;
use common::parse_to_text;

// Convert to num array where spaces are -1
fn parse_filesystem(input: &String) -> Vec<i32> {
    let mut is_space = false;
    let mut number: i32 = 0;
    let mut file_system: Vec<i32> = Vec::new();

    for char in input.chars() {
        let size: usize = char.to_digit(10).unwrap() as usize;
        if !is_space {
            for _ in 0..size {
                file_system.push(number);
            }
            number += 1;
        } else {
            for _ in 0..size {
                file_system.push(-1);
            }
        }
        is_space = !is_space;
    }

    file_system
}

// Sort last i32 into first space
fn defrag_system(filesystem: &Vec<i32>) -> Vec<i32> {
    let mut defragged: Vec<i32> = Vec::new();
    let mut count_from_end = 1; // for scanning
    let mut swap_count = 0; // for truncating

    for data_block in filesystem {
        if *data_block != -1 { // if data, push
            defragged.push(*data_block);
        } else { // if not data
            let mut found = false;

            while count_from_end <= filesystem.len() && !found { // seek from end until data found
                let next = filesystem[filesystem.len() - count_from_end];
                if next != -1 { // if data, push to output
                    defragged.push(next);
                    found = true;
                    swap_count += 1;
                }
                count_from_end += 1;
            }
        }
    }

    defragged.truncate(defragged.len() - swap_count); // 0, -1, 2 becomes 0, 2, 2. Remove copied data at end
    defragged
}

// Get a vector slice for the start/end of a data block to move
fn get_last_to_move(file_number: i32, input: &mut Vec<i32>) -> (i32, i32) {
    let mut start_index = -1;
    let mut end_index = -1;

    for (i, &number) in input.iter().enumerate() {
        if number == file_number {
            if start_index == -1 {
                start_index = i as i32;
            }
            end_index = i as i32 + 1;
        }
    }

    (start_index, end_index) // -1, -1 if none found
}

// Same as the above but go left to right and find first slice big enough to fit size
fn get_first_space_to_fit(size: i32, max_index: i32, input: &mut Vec<i32>) -> (i32, i32) {
    let mut count = 0;
    let mut start_index: Option<usize> = None;

    for (i, number) in input.iter().enumerate() {
        if i as i32 == max_index {
            break;
        }
        if *number == -1 {
            if start_index.is_none() {
                start_index = Some(i);
            }
            count += 1;
        } else {
            start_index = None;
            count = 0;
        }

        if count >= size {
            return (start_index.unwrap() as i32, (start_index.unwrap() as i32 + size));
        }
    }

    (-1, -1) // -1, -1 if none found
}

fn swap_slices(vec: &mut Vec<i32>, start1: usize, end1: usize, start2: usize, end2: usize) {
    let slice1 = &vec[start1..end1];
    let slice2 = &vec[start2..end2];

    let mut swapped_vec = Vec::new();

    swapped_vec.extend(&vec[..start1]);
    swapped_vec.extend(slice2.iter().copied());
    swapped_vec.extend(&vec[end1..start2]);
    swapped_vec.extend(slice1.iter().copied());
    swapped_vec.extend(&vec[end2..]);

    *vec = swapped_vec;
}

fn block_defrag_system(input: &mut Vec<i32>) -> Vec<i32> {
    // Work from last file (biggest numbered file)
    let mut file_number = *input.iter().max().unwrap();

    loop {
        // Slice (block) for file number
        let (block_start, block_end) = get_last_to_move(file_number, input);

        if block_start == -1 && block_end == -1 {
            break;
        }

        let block_size = block_end - block_start;
        // Check from 0 to start of block for a space
        let (space_start, space_end) = get_first_space_to_fit(block_size, block_start, input);

        // If found, swap
        if space_start != -1 {
            swap_slices(input, space_start as usize, space_end as usize, block_start as usize, block_end as usize);
        }

        file_number -= 1;
        if file_number < 0 {
            break
        }
    }

    input.clone()
}

fn generate_checksum(defragged_system: &Vec<i32>) -> i64 {
    let mut result = 0;

    for (index, id) in defragged_system.iter().enumerate() {
        if *id == -1 {
            continue;
        }
        result += (index as i32 * id) as i64;
    }

    result
}

fn main() {
    let input = parse_to_text("day09part01.txt").unwrap();
    let mut filesystem = parse_filesystem(&input);
    let defragged_filesystem = defrag_system(&filesystem);

    let part_one = generate_checksum(&defragged_filesystem);
    println!("{}", part_one);

    let block_defragged_filesystem = block_defrag_system(&mut filesystem);
    let part_two = generate_checksum(&block_defragged_filesystem);
    println!("{}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defrag_test() {
        let input = "2333133121414131402".to_string();
        let filesystem = parse_filesystem(&input);

        assert_eq!(
            defrag_system(&filesystem),
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6
            ]
        );
    }

    #[test]
    fn block_defrag_test() {
        let input = "2333133121414131402".to_string();
        let filesystem = &mut parse_filesystem(&input);

        assert_eq!(
            block_defrag_system(filesystem),
            vec![
                0,0,9,9,2,1,1,1,7,7,7,-1,4,4,-1,3,3,3,-1,-1,-1,-1,5,5,5,5,-1,6,6,6,6,-1,-1,-1,-1,-1,8,8,8,8,-1,-1
            ]
        );
    }

    #[test]
    fn part_one_test() {
        let input = "2333133121414131402".to_string();
        let defragged_filesystem = defrag_system(&parse_filesystem(&input));

        assert_eq!(generate_checksum(&defragged_filesystem), 1928);
    }

    #[test]
    fn part_two_test() {
        let input = "2333133121414131402".to_string();
        let defragged_filesystem = block_defrag_system(&mut parse_filesystem(&input));

        assert_eq!(generate_checksum(&defragged_filesystem), 2858);
    }
}
