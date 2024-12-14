use std::collections::HashSet;

pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();
    let puzzle_input = parse_puzzle_input(&input);
    let solution_part_one = solve_part_one(puzzle_input.clone()).to_string();
    let solution_part_two = solve_part_two(puzzle_input.clone()).to_string();
    (solution_part_one.to_string(), solution_part_two.to_string())
}

fn solve_part_one(mut puzzle: Vec<i32>) -> i64 {
    let mut left_index = 0;
    let mut right_index = puzzle.len() - 1;

    loop {
        if right_index <= left_index {
            break;
        }

        if puzzle[left_index] != -1 {
            left_index += 1;
            continue;
        }

        if puzzle[right_index] == -1 {
            right_index -= 1;
            continue;
        }

        puzzle.swap(left_index, right_index);
        left_index += 1;
        right_index -= 1;
    }

    calc_checksum(&puzzle)
}

fn solve_part_two(mut puzzle: Vec<i32>) -> i64 {
    let mut file_end_index = puzzle.len() - 1;
    let mut file_start_index: usize;

    let mut visited_file_blocks: HashSet<i32> = HashSet::new();

    'outer: loop {
        // Find start and end index of a data blocks
        while file_end_index > 0 && puzzle[file_end_index] == -1 {
            file_end_index -= 1;
        }
        file_start_index = file_end_index;
        while file_start_index > 0 && puzzle[file_start_index - 1] == puzzle[file_start_index] {
            file_start_index -= 1;
        }
        let file_block_size = file_end_index - file_start_index + 1;

        // Ignore data blocks that already have been visited
        if visited_file_blocks.contains(&puzzle[file_start_index]) {
            file_end_index -= file_block_size;
            continue 'outer;
        } else {
            visited_file_blocks.insert(puzzle[file_start_index]);
        }

        // Match first free
        let mut free_start_index = 0;
        let mut free_end_index: usize;
        'find_free_first: loop {
            while free_start_index < file_start_index && puzzle[free_start_index] != -1 {
                free_start_index += 1;
            }
            free_end_index = free_start_index;
            while free_end_index < file_start_index && puzzle[free_end_index + 1] == -1 {
                free_end_index += 1;
            }
            let free_size = free_end_index - free_start_index + 1;

            // Check if space is large enough and swap in case of
            if free_size >= file_block_size {
                for swap_index in 0..file_block_size {
                    puzzle.swap(free_start_index + swap_index, file_start_index + swap_index);
                }
                break 'find_free_first;
            } else {
                free_start_index += free_size;
            }

            if free_start_index >= file_start_index || file_start_index == 0 && file_block_size == 0
            {
                break 'find_free_first;
            }
        }

        if file_end_index <= file_block_size {
            break;
        }
        file_end_index -= file_block_size;
    }
    calc_checksum(&puzzle)
}

fn calc_checksum(puzzle: &Vec<i32>) -> i64 {
    let mut check_sum: i64 = 0;
    for i in 0..puzzle.len() {
        if puzzle[i] != -1 {
            check_sum += puzzle[i] as i64 * i as i64;
        }
    }
    check_sum
}

fn parse_puzzle_input(input: &str) -> Vec<i32> {
    let input = input.lines().next().unwrap();
    let disk_size = input
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |acc, byte| acc + byte);

    let mut file_index = 0;
    let mut output = Vec::with_capacity(disk_size as usize);
    for (index, digit) in input.chars().enumerate() {
        let is_free_space = index % 2 != 0;
        for _ in 0..digit.to_digit(10).unwrap() {
            if is_free_space {
                output.push(-1);
            } else {
                output.push(file_index);
            }
        }
        if !is_free_space {
            file_index += 1;
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_one() {
        let input = "2333133121414131402".to_ascii_lowercase();
        let puzzle_input = parse_puzzle_input(&input);
        let solution = solve_part_one(puzzle_input.clone()).to_string();
        assert_eq!(solution, "1928");
    }
    #[test]
    fn test_solve_part_two() {
        let input = "2333133121414131402".to_ascii_lowercase();
        let puzzle_input = parse_puzzle_input(&input);
        let solution = solve_part_two(puzzle_input.clone()).to_string();
        assert_eq!(solution, "2858");
    }
}
