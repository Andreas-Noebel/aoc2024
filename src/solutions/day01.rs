use std::collections::HashMap;
use std::fs;

pub fn solve(puzzle_file_path: &str) -> (String, String) {
    let solution_one  = solve_part_one(puzzle_file_path);
    let solutions_two = solve_part_two(puzzle_file_path);
    (solution_one, solutions_two)
}

fn solve_part_one(file_path: &str) -> String {
    let (mut left_list,mut right_list) = parse_file(file_path);
    left_list.sort();
    right_list.sort();

    let mut result = 0;
    for i in 0..left_list.len() {
        result += (left_list[i] - right_list[i]).abs();
    }
    result.to_string()
}

fn solve_part_two(file_path: &str) -> String{
    let (left_list, right_list) = parse_file(file_path);
    let mut numb_occurrence = HashMap::new();

    for right_element in right_list {
        *numb_occurrence.entry(right_element).or_insert(0) += 1;
    }
    let mut sum = 0;
    for left_element in left_list {
        sum += left_element * numb_occurrence.get(&left_element).or(Some(&0)).unwrap();
    }
    sum.to_string()
}

fn parse_file(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let file_content = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = file_content.split("\n").filter(|x| !x.is_empty()).collect();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in lines {
        let line_split = line.split_whitespace().collect::<Vec<&str>>();
        let left_list_item = line_split[0].parse::<i32>().unwrap();
        let right_list_item = line_split[1].parse::<i32>().unwrap();

        left_list.push(left_list_item);
        right_list.push(right_list_item);
    }

    (left_list, right_list)
}
