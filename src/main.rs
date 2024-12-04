pub mod solutions;

use std::path::Path;
use ansi_term::Style;
use std::time::Instant;

macro_rules! solution {
    ($day:tt) => {{
        let day = stringify!($day);
        let input_file = Path::new("resources").join(day).join("input.txt");
        if(!input_file.exists()) {
            panic!("Input file {} for {} does not exist!", input_file.display(), day);
        }
        print_solution(day, solutions::$day::solve(input_file.to_str().unwrap()));
    }}
}

fn main() {
    println!("{}", Style::new().bold().paint("---- Advent of Rust 2024 🦀🎄⭐   ----"));
    let timer = Instant::now();
    solution!(day01);
    solution!(day02);
    solution!(day03);
    solution!(day04);
    let time = timer.elapsed();
    println!("Total runtime: {:.2?}", time);
}

fn print_solution(day: &str, answers: (String, String)) {
    println!("{}", day);
    println!("  ├─── Part 1: {}", answers.0);
    println!("  └─── Part 2: {}", answers.1);
    println!("-------------------------------------")
}
