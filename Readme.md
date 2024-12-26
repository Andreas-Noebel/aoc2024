# AOC 2024 Rust 🦀⭐🎄

## 🚀 Getting started

Clone this repository and build the binary yourself or download a prebuilt one from the release tab

``` bash
git clone --depth 1 https://github.com/Andreas-Noebel/aoc2024
cd aoc2024
cargo build --release  
```

## 🧩 Puzzle Inputs

You may notice that sample inputs and puzzle inputs from Advent of Code are not included in this repository. This is
intentional for the following reasons:

1. **Copyright Concerns:** The inputs provided by Advent of Code are copyrighted, and redistributing them directly
   violates their terms of service.

2. **Personalized Inputs:** The puzzle inputs are unique to each participant, meaning my input files may not be relevant
   or useful for your solutions.

If you’d like to run these solutions, you can copy your own inputs from
the [Advent of Code](https://adventofcode.com/2024/) website and place them in the appropriate directory.

**Input directory structure**

```
./resources
    │
    ├───day01
    │     ├─ example.txt
    │     └─ input.txt
    │
    ├───day02
    │     ├─ example.txt
    │     └─ input.txt
    │
   ...
```

**Alternatively** you can use the bash script `download_puzzles.sh` in this repository which requires a session key of
your AoC account. For the sake of security, please check the script before you run it.

``` bash
./download_puzzles.sh
```

## 📖 Usage

``` bash
# With the binary
./aoc2024 [Options]

# or directly with cargo
cargo run --release -- [Options]
```

**Options**

``` bash
-d, -day  N          # Solves a specific day N where N is in [1..25] 
-a, -all             # Solves all days
-b, -benchmark N     # Runs the internal benchmark N times
-h, -help            # Prints this page      
```

## 🐌 Benchmark

Benchmarks are made with the internal benchmark command, which runs each solver 10 times sequentially.

Hardware: Intel Core i5-1135G7 @ 2.40Ghz

Cargo Build-Flags: `--release`

| Day | Links                                                                                | Average [µs] |  Min [µs] |  Max [µs] |
|-----|--------------------------------------------------------------------------------------|-------------:|----------:|----------:|
| 01  | [Puzzle](https://adventofcode.com/2024/day/1), [Solution](./src/solutions/day01.rs)  |          524 |       434 |       859 |
| 02  | [Puzzle](https://adventofcode.com/2024/day/2), [Solution](./src/solutions/day02.rs)  |          729 |       618 |     1 480 |
| 03  | [Puzzle](https://adventofcode.com/2024/day/3), [Solution](./src/solutions/day03.rs)  |        1 135 |     1 043 |     1 509 |
| 04  | [Puzzle](https://adventofcode.com/2024/day/4), [Solution](./src/solutions/day04.rs)  |        1 038 |       911 |     1 390 |
| 05  | [Puzzle](https://adventofcode.com/2024/day/5), [Solution](./src/solutions/day05.rs)  |        1 528 |     1 381 |     2 236 |
| 06  | [Puzzle](https://adventofcode.com/2024/day/6), [Solution](./src/solutions/day06.rs)  |    2 522 967 | 2 480 235 | 2 618 794 |
| 07  | [Puzzle](https://adventofcode.com/2024/day/7), [Solution](./src/solutions/day06.rs)  |      958 841 |   942 852 |   978 263 |
| 08  | [Puzzle](https://adventofcode.com/2024/day/8), [Solution](./src/solutions/day08.rs)  |        5 038 |     4 739 |     5 461 |
| 09  | [Puzzle](https://adventofcode.com/2024/day/9), [Solution](./src/solutions/day09.rs)  |      105 654 |   102 882 |   109 044 |
| 10  | [Puzzle](https://adventofcode.com/2024/day/10), [Solution](./src/solutions/day10.rs) |          797 |       750 |     1 010 |
| 11  | [Puzzle](https://adventofcode.com/2024/day/11), [Solution](./src/solutions/day11.rs) |      853 943 |   831 354 |   915 658 |
| 12  | [Puzzle](https://adventofcode.com/2024/day/12), [Solution](./src/solutions/day12.rs) |        9 303 |     8 653 |    10 120 |
| 13  | [Puzzle](https://adventofcode.com/2024/day/13), [Solution](./src/solutions/day13.rs) |          522 |       475 |       689 |
| 14  | [Puzzle](https://adventofcode.com/2024/day/14), [Solution](./src/solutions/day14.rs) |      922 642 |   901 249 |   953 789 |
| 15  | [Puzzle](https://adventofcode.com/2024/day/15), [Solution](./src/solutions/day15.rs) |       55 854 |    53 142 |    58 544 |
| 16  | [Puzzle](https://adventofcode.com/2024/day/16), [Solution](./src/solutions/day16.rs) |      155 281 |   143 279 |   166 694 |
| 17  | [Puzzle](https://adventofcode.com/2024/day/17), [Solution](./src/solutions/day17.rs) |          172 |       134 |       350 |
| 18  | [Puzzle](https://adventofcode.com/2024/day/18), [Solution](./src/solutions/day18.rs) |        4 261 |     4 056 |     4 541 |
| 19  | [Puzzle](https://adventofcode.com/2024/day/19), [Solution](./src/solutions/day19.rs) |       78 145 |    76 200 |    80 449 |
| 20  | [Puzzle](https://adventofcode.com/2024/day/20), [Solution](./src/solutions/day20.rs) |      799 147 |   752 342 | 1 013 325 |
| 21  | [Puzzle](https://adventofcode.com/2024/day/21), [Solution](./src/solutions/day21.rs) |        1 546 |     1 342 |     1 883 |
| 22  | [Puzzle](https://adventofcode.com/2024/day/22), [Solution](./src/solutions/day22.rs) |    6 353 562 |  6171 416 | 6 696 485 |
| 23  | [Puzzle](https://adventofcode.com/2024/day/23), [Solution](./src/solutions/day23.rs) |      740 783 |   728 355 |   749 630 |
| 24  | [Puzzle](https://adventofcode.com/2024/day/24), [Solution](./src/solutions/day24.rs) |        3 398 |     3 003 |     4 353 |
| 25  | [Puzzle](https://adventofcode.com/2024/day/25), [Solution](./src/solutions/day25.rs) |          894 |       593 |     1 501 |

