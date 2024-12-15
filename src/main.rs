pub mod solutions;

use std::env;
use std::path::Path;
use std::time::Instant;

macro_rules! solution {
    ($day:tt) => {{
        let day = stringify!($day);
        let input_file = Path::new("resources").join(day).join("input.txt");
        if (!input_file.exists()) {
            let error_string = format!("File {} doesn't exist", input_file.to_str().unwrap());
            return Err(error_string);
        }
        let solution: (String, String) = solutions::$day::solve(input_file.to_str().unwrap());
        return Ok(solution);
    }};
}

const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_RESET: &str = "\x1b[0m";
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let ci_options = if args.len() > 1 {
        parse_options(&args)
    } else {
        CIOptions {
            target: ExecutionTarget::RunSingleDay(15),
        }
    };
    println!("{ANSI_BOLD}---- Advent of Rust 2024 🦀🎄⭐   ----{ANSI_RESET}");
    execute_ci_options(ci_options);
}

fn print_solution(day: &str, answers: (String, String), time: String) {
    println!("Day {} [{}]", day, time);
    println!("  ├─── Part 1: {}", answers.0);
    println!("  └─── Part 2: {}", answers.1);
    println!("-------------------------------------")
}

struct CIOptions {
    target: ExecutionTarget,
}
enum ExecutionTarget {
    RunAllDays,
    RunSingleDay(i32),
    Help,
}

fn execute_ci_options(options: CIOptions) {
    match options.target {
        ExecutionTarget::RunSingleDay(day) => {
            let timer = Instant::now();
            match solve_day(day) {
                Ok(solution) => {
                    let elapsed_time = format!("{:.2?}", timer.elapsed());
                    print_solution(day.to_string().as_str(), solution, elapsed_time);
                }
                Err(_reason) => {
                    eprintln!("{}", _reason);
                }
            }
        }
        ExecutionTarget::RunAllDays => {
            let timer = Instant::now();
            for day in 0..15 {
                let lap_time = Instant::now();
                match solve_day(day) {
                    Ok(solution) => {
                        let elapsed_time = format!("{:.2?}", lap_time.elapsed());
                        print_solution(day.to_string().as_str(), solution, elapsed_time);
                    }
                    Err(_reason) => {}
                }
            }
            println!("Total runtime: {:.2?}", timer.elapsed());
        }
        ExecutionTarget::Help => {}
    }
}
fn parse_options(args: &Vec<String>) -> CIOptions {
    let mut args = args.iter().peekable();
    let _ = args.next().unwrap();

    let mut execution_target = ExecutionTarget::Help;

    while let Some(_) = args.peek() {
        let command = args.next().unwrap();
        match command.as_ref() {
            "-a" | "-all" => {
                execution_target = ExecutionTarget::RunAllDays;
            }
            "-d" | "-day" => match args.next() {
                Some(day) => {
                    let d = day.parse::<i32>().unwrap();
                    execution_target = ExecutionTarget::RunSingleDay(d);
                }
                None => {}
            },
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }

    CIOptions {
        target: execution_target,
    }
}

fn solve_day(day: i32) -> Result<(String, String), String> {
    let solution: Result<(String, String), String> = match day {
        1 => solution!(day01),
        2 => solution!(day02),
        3 => solution!(day03),
        4 => solution!(day04),
        5 => solution!(day05),
        6 => solution!(day06),
        7 => solution!(day07),
        8 => solution!(day08),
        9 => solution!(day09),
        10 => solution!(day10),
        11 => solution!(day11),
        12 => solution!(day12),
        13 => solution!(day13),
        14 => solution!(day14),
        15 => solution!(day15),
        _ => Err("Unknown day".to_string()),
    };
    solution
}
