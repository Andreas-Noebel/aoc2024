# AOC 2024 Rust 🦀⭐🎄

## 🚀 Getting started

### Executable

Clone this repository and build the binary yourself or download a prebuilt one from the release tab

``` bash
git clone --depth 0 https://github.com/Andreas-Noebel/aoc2024
cd aoc2024
cargo build --release  
```

### 🧩 Puzzle Inputs

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
-d, -day  <n>       # Solves a specific day n where n is in [1..25] 
-a, -all            # Solves all days
-b, -benchmark      # Runs the internal benchmark
-h, -help           # Prints this page      
```

## 🐌 Benchmark

Benchmarks are made with the internal benchmark command, which runs each solver 10 times sequentially.

Hardware: Intel Core i5-1135G7 @ 2.40Ghz

Cargo Build-Flags: `--release`

| Day | Puzzle                                         | Average   | Min       | Max       |
|-----|------------------------------------------------|-----------|-----------|-----------|
| 0   | [Puzzle](https://adventofcode.com/2024/day/1)  | 580.00 ns | 100.00 ns | 4.70 µs   |
| 1   | [Puzzle](https://adventofcode.com/2024/day/2)  | 498.09 µs | 441.90 µs | 762.80 µs |
| 2   | [Puzzle](https://adventofcode.com/2024/day/3)  | 671.00 µs | 634.40 µs | 704.90 µs |
| 3   | [Puzzle](https://adventofcode.com/2024/day/4)  | 1.30 ms   | 1.11 ms   | 1.60 ms   |
| 4   | [Puzzle](https://adventofcode.com/2024/day/5)  | 968.16 µs | 873.40 µs | 1.11 ms   |
| 5   | [Puzzle](https://adventofcode.com/2024/day/6)  | 1.32 ms   | 1.26 ms   | 1.46 ms   |
| 6   | [Puzzle](https://adventofcode.com/2024/day/7)  | 2.56 s    | 2.44 s    | 3.08 s    |
| 7   | [Puzzle](https://adventofcode.com/2024/day/8)  | 903.14 ms | 894.36 ms | 913.98 ms |
| 8   | [Puzzle](https://adventofcode.com/2024/day/9)  | 5.02 ms   | 4.78 ms   | 5.46 ms   |
| 9   | [Puzzle](https://adventofcode.com/2024/day/10) | 105.92 ms | 103.44 ms | 115.09 ms |
| 10  | [Puzzle](https://adventofcode.com/2024/day/11) | 861.14 µs | 764.60 µs | 1.04 ms   |
| 11  | [Puzzle](https://adventofcode.com/2024/day/12) | 813.64 ms | 801.40 ms | 827.65 ms |
| 12  | [Puzzle](https://adventofcode.com/2024/day/13) | 8.71 ms   | 8.44 ms   | 9.28 ms   |
| 13  | [Puzzle](https://adventofcode.com/2024/day/14) | 521.73 µs | 465.60 µs | 697.00 µs |
| 14  | [Puzzle](https://adventofcode.com/2024/day/15) | 904.41 ms | 897.88 ms | 910.41 ms |
| 15  | [Puzzle](https://adventofcode.com/2024/day/16) | 55.80 ms  | 54.90 ms  | 56.93 ms  |
| 16  | [Puzzle](https://adventofcode.com/2024/day/17) | 153.08 ms | 144.50 ms | 168.89 ms |
| 17  | [Puzzle](https://adventofcode.com/2024/day/18) | 169.37 µs | 128.00 µs | 316.50 µs |
| 18  | [Puzzle](https://adventofcode.com/2024/day/19) | 4.22 ms   | 4.02 ms   | 4.44 ms   |
| 19  | [Puzzle](https://adventofcode.com/2024/day/20) | 77.68 ms  | 76.99 ms  | 78.95 ms  |
| 20  | [Puzzle](https://adventofcode.com/2024/day/21) | 753.41 ms | 747.19 ms | 774.52 ms |
| 21  | [Puzzle](https://adventofcode.com/2024/day/22) | 1.56 ms   | 1.34 ms   | 2.11 ms   |
| 22  | [Puzzle](https://adventofcode.com/2024/day/23) | 6.09 s    | 6.04 s    | 6.26 s    |
| 23  | [Puzzle](https://adventofcode.com/2024/day/24) | 732.64 ms | 705.17 ms | 853.47 ms |
| 24  | [Puzzle](https://adventofcode.com/2024/day/25) | 3.11 ms   | 2.97 ms   | 3.81 ms   |
| 25  | [Puzzle](https://adventofcode.com/2024/day/26) | 601.63 µs | 527.50 µs | 730.30 µs |
