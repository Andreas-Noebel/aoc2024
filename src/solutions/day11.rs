use std::collections::HashMap;

pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();

    let input = input
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let solution_part_one = solve_part_one(&input);
    let solution_part_two = solve_part_two(&input);
    (solution_part_one, solution_part_two)
}

fn solve_part_one(input: &Vec<u64>) -> String {
    let mut shortcuts: HashMap<(u64, u8), u64> = HashMap::new();
    let solution: u64 = input.iter().map(|x| blink(*x, 25, &mut shortcuts)).sum();
    solution.to_string()
}

fn solve_part_two(input: &Vec<u64>) -> String {
    let mut shortcuts: HashMap<(u64, u8), u64> = HashMap::new();

    generate_shortcuts(&mut shortcuts, 75);
    let solution: u64 = input
        .iter()
        .map(|x| {
            return blink(*x, 75, &mut shortcuts);
        })
        .sum();
    solution.to_string()
}

fn blink(stone: u64, depth: u8, shortcuts: &mut HashMap<(u64, u8), u64>) -> u64 {
    if depth == 0 {
        return 1;
    }

    if stone < 10 {
        if shortcuts.contains_key(&(stone, depth)) {
            return shortcuts[&(stone, depth)];
        }
    }

    if stone == 0 {
        return blink(1, depth - 1, shortcuts);
    }

    let stone_string = stone.to_string();
    let has_even_digits = stone_string.len() % 2 == 0;

    if has_even_digits {
        let (left, right) = stone_string.split_at(stone_string.len() / 2);
        let left = left.parse::<u64>().unwrap();
        let right = right.parse::<u64>().unwrap();
        let left = blink(left, depth - 1, shortcuts);
        let right = blink(right, depth - 1, shortcuts);
        return left + right;
    }

    blink(stone * 2024, depth - 1, shortcuts)
}

fn generate_shortcuts(target: &mut HashMap<(u64, u8), u64>, depth: u8) {
    for d in 1..depth + 1 {
        for stone in 0..10 {
            let solution = blink(stone, d, target);
            target.insert((stone, d), solution);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day11/example.txt").unwrap();

        let input = input
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u64>>();

        let solution = solve_part_one(&input);
        assert_eq!(solution, "55312");
    }
}
