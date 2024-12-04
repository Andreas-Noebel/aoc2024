use regex::Regex;
use std::fs;

pub fn solve(puzzle_file_path: &str) -> (String, String) {
    let input = fs::read_to_string(puzzle_file_path).unwrap();
    let instructions = parse_instructions(&input);
    let solution_one = solve_part_one(&instructions);
    let solution_two = solve_part_two(&instructions);
    (solution_one, solution_two)
}
enum Instruction {
    MUL(i32, i32),
    DONT,
    DO,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let reg = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").unwrap();
    let instructions = reg
        .find_iter(input)
        .map(|needle| needle.as_str())
        .map(|ins_match| {
            if ins_match.contains("don") {
                return Instruction::DONT;
            }
            if ins_match.contains("do") {
                return Instruction::DO;
            }
            if ins_match.contains("mul") {
                let factors = ins_match
                    .replace("mul(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|f| f.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                return Instruction::MUL(factors[0], factors[1]);
            }
            panic!("Unknown instruction: {}", ins_match)
        })
        .collect::<Vec<Instruction>>();
    instructions
}

// 161289189
fn solve_part_one(instructions: &Vec<Instruction>) -> String {
    let mut sum = 0;
    for instruction in instructions {
        match instruction {
            Instruction::MUL(f1, f2) => sum += f1 * f2,
            _ => {}
        }
    }
    sum.to_string()
}

// 83595109
fn solve_part_two(instructions: &Vec<Instruction>) -> String {
    let mut is_do_enabled = true;
    let mut line_sum = 0;
    for instruction in instructions {
        match instruction {
            Instruction::DO => is_do_enabled = true,
            Instruction::DONT => is_do_enabled = false,
            Instruction::MUL(f1, f2) => {
                if is_do_enabled {
                    line_sum += f1 * f2
                }
            }
        }
    }
    line_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./resources/day03/example.txt").unwrap();
        let puzzle = parse_instructions(&input);
        let solution_one = solve_part_one(&puzzle);
        assert_eq!(solution_one, "161");
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./resources/day03/example.txt").unwrap();
        let puzzle = parse_instructions(&input);
        let solution_two = solve_part_two(&puzzle);
        assert_eq!(solution_two, "48");
    }
}
