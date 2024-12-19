use std::collections::HashMap;

pub fn solve(input_file: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file).unwrap();
    let (towels, target) = parse_puzzle(&*input);
    let solution_one = solve_part_one(&towels, &target);
    let solution_two = solve_part_two(&towels, &target);
    (solution_one.to_string(), solution_two.to_string())
}

fn parse_puzzle(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut line_iter = input.lines();
    let towels = line_iter.next().unwrap().split(", ").collect::<Vec<&str>>();
    line_iter.next();
    let targets = Vec::from_iter(line_iter);
    (towels, targets)
}

fn solve_part_one(towels: &Vec<&str>, targets: &Vec<&str>) -> i32 {
    targets
        .iter()
        .filter(|&&t| prefixes(&towels, t, &mut HashMap::new()) > 0)
        .count() as i32
}

fn solve_part_two(towels: &Vec<&str>, targets: &Vec<&str>) -> u64 {
    targets
        .iter()
        .map(|&t| prefixes(&towels, t, &mut HashMap::new()))
        .sum()
}

fn prefixes<'a>(part: &Vec<&str>, word: &'a str, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if cache.contains_key(word) {
        return cache[word];
    }
    let mut counter = 0;
    for prefix in part {
        if !word.starts_with(prefix) {
            continue;
        }
        if prefix.len() == word.len() {
            counter += 1;
        }
        counter += prefixes(part, word.strip_prefix(prefix).unwrap(), cache);
    }
    cache.insert(word, counter);
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_puzzle() {
        let input = std::fs::read_to_string("./resources/day19/example.txt").unwrap();
        let (towels, target) = parse_puzzle(&*input);
        assert_eq!(towels.len(), 8);
        assert_eq!(target.len(), 8);
    }

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day19/input.txt").unwrap();
        let (towels, target) = parse_puzzle(&*input);
        let solution = solve_part_one(&towels, &target);
        println!("{}", solution);
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day19/input.txt").unwrap();
        let (towels, target) = parse_puzzle(&*input);
        let solution = solve_part_two(&towels, &target);
        println!("{}", solution);
    }
}