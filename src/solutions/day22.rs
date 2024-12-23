use std::collections::{HashMap, HashSet};
use std::iter::zip;

pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();

    let initial_secrets: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let monkey_market = MonkeyMarket::new(&initial_secrets);
    let solution_one = solve_part_one(&monkey_market).to_string();
    let solution_two = solve_part_two(&monkey_market).to_string();

    (solution_one, solution_two)
}

fn solve_part_one(monkey_market: &MonkeyMarket) -> u64 {
    monkey_market.get_secret_numbers_at(2000).iter().sum()
}

fn solve_part_two(monkey_market: &MonkeyMarket) -> u64 {
    let patterns = monkey_market.generate_all_patterns(4);
    let cache = monkey_market.create_dictionaries();

    let mut best_revenue: u32 = 0;

    for pattern in patterns {
        // Test Pattern
        let revenue_of_pattern = cache
            .iter()
            .filter_map(|dic| dic.get(&pattern))
            .map(|x| *x as u32)
            .sum::<u32>();

        if revenue_of_pattern > best_revenue {
            best_revenue = revenue_of_pattern;
        }
    }
    best_revenue as u64
}

type Pattern = (i8, i8, i8, i8);
struct MonkeyMarket {
    initial_secret: Vec<u64>,
    prices: Vec<Vec<u8>>,
    diffs: Vec<Vec<i8>>,
}

impl MonkeyMarket {
    fn new(initial_secrets: &Vec<u64>) -> MonkeyMarket {
        let mut prices: Vec<Vec<u8>> = Vec::with_capacity(initial_secrets.len());
        let mut diffs: Vec<Vec<i8>> = Vec::with_capacity(initial_secrets.len());

        for s in initial_secrets {
            let steps = 2000;
            let mut price: Vec<u8> = Vec::with_capacity(steps);

            let mut current = *s;
            for _ in 0..steps {
                price.push((current % 10) as u8);
                let next_secret = next_secret_number(current);
                current = next_secret;
            }

            let local_diffs = get_price_diffs(&price);

            prices.push(price);
            diffs.push(local_diffs)
        }

        MonkeyMarket {
            initial_secret: initial_secrets.clone(),
            prices,
            diffs,
        }
    }

    fn get_secret_numbers_at(&self, steps: usize) -> Vec<u64> {
        self.initial_secret
            .iter()
            .map(|initial_secret| {
                let mut current = *initial_secret;
                for _ in 0..steps {
                    let next_secret = next_secret_number(current);
                    current = next_secret;
                }
                current
            })
            .collect::<Vec<u64>>()
    }

    fn generate_all_patterns(&self, pattern_length: i8) -> HashSet<Pattern> {
        let mut unique_patterns: HashSet<Pattern> = HashSet::new();

        for monkey_index in 0..self.initial_secret.len() {
            let diffs: &Vec<i8> = &self.diffs[monkey_index];
            for i in 0..diffs.len() - pattern_length as usize {
                let pattern = (diffs[i], diffs[i + 1], diffs[i + 2], diffs[i + 3]);
                if !unique_patterns.contains(&pattern) {
                    unique_patterns.insert(pattern);
                }
            }
        }

        unique_patterns
    }

    fn create_dictionaries(&self) -> Vec<HashMap<Pattern, u8>> {
        let mut dictionaries: Vec<HashMap<Pattern, u8>> = Vec::new();
        let pattern_length = 4;

        for i in 0..self.initial_secret.len() {
            let prices = &self.prices[i];
            let diffs = &self.diffs[i];
            let mut dic: HashMap<Pattern, u8> = HashMap::new();

            diffs
                .windows(pattern_length as usize)
                .enumerate()
                .for_each(|(index, diffs)| {
                    let prices = prices[index + pattern_length as usize];
                    let e = (diffs[0], diffs[1], diffs[2], diffs[3]);
                    if !dic.contains_key(&e) {
                        dic.insert(e, prices);
                    }
                });
            dictionaries.push(dic);
        }
        dictionaries
    }
}

fn next_secret_number(secret: u64) -> u64 {
    let c_1 = prune(mix(secret, secret * 64));
    let c_2 = prune(mix(c_1, c_1 / 32));
    let c_3 = prune(mix(c_2, c_2 * 2048));
    c_3
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn get_price_diffs(prices: &Vec<u8>) -> Vec<i8> {
    let iter_left = prices.iter();
    let mut iter_right = prices.iter();
    iter_right.next();
    zip(iter_left, iter_right)
        .map(|(x, y)| *y as i8 - *x as i8)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let monkey_market = MonkeyMarket::new(&vec![1, 10, 100, 2024]);
        let solution = solve_part_one(&monkey_market);
        assert_eq!(solution, 37327623);
    }

    #[test]
    fn test_part_two() {
        let monkey_market = MonkeyMarket::new(&vec![1, 2, 3, 2024]);
        let solution = solve_part_two(&monkey_market);
        assert_eq!(solution, 23);
    }
}
