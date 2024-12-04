pub mod solutions;

use ansi_term::Style;
use std::time::Instant;

fn main() {
    println!("{}", Style::new().bold().paint("---- Advent of Rust 2024 🦀🎄⭐   ----"));
    let timer = Instant::now();
    solve_day(4);
    let time = timer.elapsed();
    println!("Total runtime: {:.2?}", time);
}

fn solve_day(day: u8) {
    match day {
        1 => print_solution(1, solutions::day01::solve()),
        2 => print_solution(2, solutions::day02::solve()),
        3 => print_solution(3, solutions::day03::solve()),
        4 => print_solution(4, solutions::day04::solve()),
        _ => println!("Not implemented"),
    }
}

fn print_solution(day: u8, answers: (String, String)) {
    println!("Day {}", day);
    println!("  ├─── Part 1: {}", answers.0);
    println!("  └─── Part 2: {}", answers.1);
    println!("-------------------------------------")
}
