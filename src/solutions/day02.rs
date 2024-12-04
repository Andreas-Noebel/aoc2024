use std::fs;

pub fn solve(puzzle_file_path: &str) -> (String, String) {
    let puzzle = parse_file(puzzle_file_path);
    let solution_one = solve_part_one(&puzzle);
    let solution_two = solve_part_two(&puzzle);
    (solution_one, solution_two)
}

fn solve_part_one(puzzle: &Vec<Vec<i32>>) -> String {
    let safe_reports = puzzle.iter()
        .filter(|report| is_safe_report(report))
        .count();
    safe_reports.to_string()
}

fn solve_part_two(puzzle: &Vec<Vec<i32>>) -> String {
    let safe_reports = puzzle.iter()
        .filter(|report| is_safe_report_damped(report))
        .count();
    safe_reports.to_string()
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let is_report_ascending = report[0] < report[1];
    for i in 0..report.len() - 1 {
        let p1 = report[i];
        let p2 = report[i + 1];
        let is_pair_ascending = p1 < p2;
        if is_report_ascending != is_pair_ascending {
            return false;
        }
        let diff = (p1 - p2).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_safe_report_damped(report: &Vec<i32>) -> bool {
    if is_safe_report(report) {
        return true;
    }
    for drop_index in 0..report.len() {
        let mut reduced_report = report.clone();
        reduced_report.remove(drop_index);
        if is_safe_report(&reduced_report) {
            return true;
        }
    }
    false
}

fn parse_file(file_path: &str) -> Vec<Vec<i32>> {
    let lines = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line|
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect()
        ).collect::<Vec<Vec<i32>>>();
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let puzzle = parse_file("./resources/day02/example.txt");
        let solution_one = solve_part_one(&puzzle);
        assert_eq!(solution_one, "2");
    }

    #[test]
    fn test_part_two() {
        let puzzle = parse_file("./resources/day02/example.txt");
        let solution_one = solve_part_two(&puzzle);
        assert_eq!(solution_one, "4");
    }
}