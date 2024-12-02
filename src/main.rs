pub mod solutions;

use std::time::Instant;
use ansi_term::Style;

fn main() {
    println!("{}", Style::new().bold().paint("---- Advent of Rust 2024 🦀🎄⭐   ----"));
    let timer = Instant::now();
    solve_day(2);
    let time = timer.elapsed();
    println!("Total runtime: {:.2?}", time);
}

fn solve_day(day: u8) {
    match day {
        1 => print_solution(1, solutions::day01::solve()),
        2 => print_solution(2, solutions::day02::solve()),
        _ => println!("Not implemented")
    }
}

fn print_solution(day: u8, answers: (String, String)) {
    println!("Day {}", day);
    println!("  ├─── Part 1: {}", answers.0);
    println!("  └─── Part 2: {}", answers.1);
    println!("-------------------------------------")
}