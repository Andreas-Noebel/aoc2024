use std::collections::HashSet;
use std::iter::zip;

pub fn solve(input_file_path: &str) -> (String, String) {
    ("".to_string(), "".to_string())
}

fn solve_part_one(initial_secrets: &Vec<u64>) -> u64 {
    let sim = simulate(initial_secrets, 2000);
    let sum = sim.iter().sum::<u64>();
    sum
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

fn simulate(initial_secrets: &Vec<u64>, steps: u32) -> Vec<u64> {
    let mut output: Vec<u64> = vec![];
    for secret in initial_secrets.iter() {
        let mut current_secret = *secret;
        for _ in 0..steps as usize {
            current_secret = next_secret_number(current_secret);
        }
        output.push(current_secret);
    }
    output
}

fn get_prices_for_monkey(initial_secret: u64, steps: u32) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::with_capacity(steps as usize);
    let mut current_secret = initial_secret;
    for _ in 0..steps {
        output.push(current_secret % 10);
        current_secret = next_secret_number(current_secret);
    }
    output
}

fn get_price_diffs(prices: &Vec<u64>) -> Vec<i32> {
    let mut iter_left = prices.iter();
    let mut iter_right = prices.iter();
    iter_right.next();
    zip(iter_left, iter_right)
        .map(|(x, y)| *y as i32 - *x as i32)
        .collect()
}

fn evaluate_pattern(prices: &Vec<u64>, pattern: &Vec<i32>) -> u32 {
    let diffs: Vec<i32> = get_price_diffs(&prices);
    let pattern = pattern.as_slice();

    for i in 0..diffs.len() - 4 {
        let prefix = &diffs.as_slice()[i..];
        if prefix.starts_with(&pattern) {
            return prices[i + 4] as u32;
        }
    }
    0
}

fn evaluate_pattern_all(prices: &Vec<Vec<u64>>, pattern: &Vec<i32>, steps: u32) -> u32 {
    let mut total_revenue = 0;
    for price in prices.iter() {
        let revenue = evaluate_pattern(price, &pattern);
        //println!("Revenue: {}", revenue);
        total_revenue += revenue;
    }
    total_revenue
}

fn get_all_patterns(initial_secrets: &Vec<u64>, steps: u32) -> HashSet<Vec<i32>> {
    let mut unique_patterns: HashSet<Vec<i32>> = HashSet::new();
    for secret in initial_secrets.iter() {
        let prices = get_prices_for_monkey(*secret, steps);
        let diffs: Vec<i32> = get_price_diffs(&prices);
        for i in 0..diffs.len() - 4 {
            let pattern = &diffs.as_slice()[i..i + 4];
            if !unique_patterns.contains(pattern) {
                unique_patterns.insert(pattern.to_vec());
                // println!("Pattern: {:?}", pattern);
            }
        }
    }
    //println!("{:?}", unique_patterns);
    unique_patterns
}

#[cfg(test)]
mod tests {
    use crate::solutions::day22::{
        evaluate_pattern_all, get_all_patterns,
        get_prices_for_monkey,
    };

    #[test]
    fn test_solve() {
        //println!("{}", mix(42, 15));
        //println!("{}", prune(100000000));

        let input = std::fs::read_to_string("./resources/day22/input.txt").unwrap();
        let initial_secrets: Vec<u64> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();

        //let price = get_prices_for_monkey(123, 10);
        //println!("Prices {:?}", price);
        //println!("Diffs {:?}", get_price_diffs(&price));
        let mut best_price = 0;
        let mut best_pattern: Vec<i32> = vec![];
        let all_patterns = get_all_patterns(&initial_secrets, 2000);
        let all_prices = initial_secrets
            .iter()
            .map(|s| get_prices_for_monkey(*s, 2000))
            .collect::<Vec<Vec<u64>>>();
        println!("Pattern # {:?}", all_patterns.len());
        let mut current_pattern = 0;
        for pattern in all_patterns {
            //println!("{:?}", current_pattern);
            let price = evaluate_pattern_all(&all_prices, &pattern, 2000);
            if price > best_price {
                best_price = price;
                best_pattern = pattern;
            }
            current_pattern += 1;
        }
        println!("Best price: {}", best_price);
        println!("Best pattern: {:?}", best_pattern);
        //println!("{:?}", evaluate_pattern_all(&initial_secrets, &vec![-2,1,-1,3], 2000));

        /*
        for secret in initial_secrets.iter() {
            let prices = get_prices_for_monkey(*secret, 10);
            println!("{:?}", prices);
        }
        */
    }
}
