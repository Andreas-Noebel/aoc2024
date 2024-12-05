use std::collections::{HashMap, HashSet};

pub fn solve(puzzle_input_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(puzzle_input_path).unwrap();
    let (c, u) = parse_puzzle(&input);
    let solution_one = solve_part_one(&c, &u);
    let solution_two = solve_part_two(&c, &u);
    (solution_one.to_string(), solution_two.to_string())
}

fn solve_part_one(constraints: &HashMap<u8, HashSet<u8>>, updates: &Vec<Vec<u8>>) -> String {
    let solution: i32 = updates
        .iter()
        .filter(|page_update| is_page_order_correct(page_update, &constraints))
        .map(|page_update| page_update[page_update.len() / 2] as i32)
        .sum();
    solution.to_string()
}

fn solve_part_two(constraints: &HashMap<u8, HashSet<u8>>, updates: &Vec<Vec<u8>>) -> String {
    let solution: i32 = updates
        .iter()
        .filter(|page_update| !is_page_order_correct(page_update, &constraints))
        .map(|page| fix_page_order(page, &constraints))
        .map(|page_update| page_update[page_update.len() / 2] as i32)
        .sum();

    solution.to_string()
}

fn is_page_order_correct(page_order: &Vec<u8>, constraints: &HashMap<u8, HashSet<u8>>) -> bool {
    let mut page_order_rev = page_order.clone();
    page_order_rev.reverse();
    for i in 0..page_order.len() {
        match constraints.get(&page_order_rev[i]) {
            Some(page_constraints) => {
                for j in i + 1..page_order_rev.len() {
                    if page_constraints.contains(&page_order_rev[j]) {
                        return false;
                    }
                }
            }
            None => {}
        }
    }
    true
}

fn fix_page_order(page_order: &Vec<u8>, constraints: &HashMap<u8, HashSet<u8>>) -> Vec<u8> {
    let mut page_order_rev = page_order.clone();
    page_order_rev.reverse();
    for i in 0..page_order_rev.len() {
        loop {
            let mut had_swap = false;
            match constraints.get(&page_order_rev[i]) {
                Some(page_constraints) => {
                    for j in i + 1..page_order_rev.len() {
                        if page_constraints.contains(&page_order_rev[j]) {
                            page_order_rev.swap(i, j);
                            had_swap = true;
                        }
                    }
                }
                None => {}
            }
            if !had_swap {
                break;
            }
        }
    }
    page_order_rev.reverse();
    page_order_rev
}

fn parse_puzzle(input: &str) -> (HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>) {
    // Parse constraints
    let mut line_iter = input.lines();
    let mut constraint_map: HashMap<u8, HashSet<u8>> = HashMap::new();
    loop {
        let line = line_iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let line = line.split("|").collect::<Vec<&str>>();
        let c1 = line[0].parse::<u8>().unwrap();
        let c2 = line[1].parse::<u8>().unwrap();
        let constraint_set = constraint_map.entry(c1).or_insert(HashSet::new());
        constraint_set.insert(c2);
    }

    // Parse page updates
    let mut page_updates: Vec<Vec<u8>> = Vec::new();
    loop {
        match line_iter.next() {
            Some(line) => {
                if line.is_empty() {
                    break;
                }
                let update = line
                    .split(",")
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();
                page_updates.push(update);
            }
            None => break,
        }
    }
    (constraint_map, page_updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day05/example.txt").unwrap();
        let (c, u) = parse_puzzle(&input);
        let s = solve_part_one(&c, &u);
        assert_eq!(s, "143")
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day05/example.txt").unwrap();
        let (c, u) = parse_puzzle(&input);
        let s = solve_part_two(&c, &u);
        //assert_eq!(s, "5466")
        assert_eq!(s, "123")
    }
}
