type Equation = (i64, Vec<i64>);
type Puzzle = Vec<Equation>;
type Operation = fn(i64, i64) -> i64;
pub fn solve(puzzle_input_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(puzzle_input_path).unwrap();
    let puzzle = parse_puzzle(&input);
    let solution_one = solve_part_one(&puzzle);
    let solution_two = solve_part_two(&puzzle);
    (solution_one, solution_two)
}

fn solve_part_one(puzzle: &Puzzle) -> String {
    let mut solution = 0;
    let op: Vec<Operation> = vec![|x, y| x * y, |x, y| x + y];
    for equation in puzzle {
        if is_equation_solvable(&equation, &op) {
            solution += equation.0;
        }
    }
    solution.to_string()
}

fn is_equation_solvable((solution, values): &Equation, ops: &Vec<Operation>) -> bool {
    if values.len() == 1 {
        return values[0] == *solution;
    }

    // Optimization: assumes that there are no operations which reduce the result size
    if values[0] > *solution {
        return false;
    }

    for operation in ops {
        let result = operation(values[0], values[1]);

        let mut reduced_values: Vec<i64> = Vec::new();
        reduced_values.push(result);
        values.iter().skip(2).for_each(|v| reduced_values.push(*v));
        let recursive_result = is_equation_solvable(&(*solution, reduced_values), ops);
        if recursive_result {
            return true;
        }
    }
    false
}

fn solve_part_two(puzzle: &Puzzle) -> String {
    let mut solution = 0;
    let op: Vec<fn(i64, i64) -> i64> = vec![|x, y| x * y, |x, y| x + y, |x, y| {
        let mut result_concat = String::new();
        result_concat.push_str(x.to_string().as_str());
        result_concat.push_str(y.to_string().as_str());
        result_concat.parse::<i64>().unwrap()
    }];
    for equation in puzzle {
        if is_equation_solvable(&equation, &op) {
            solution += equation.0;
        }
    }
    solution.to_string()
}

fn parse_puzzle(input: &str) -> Puzzle {
    let lines = input.lines();
    let puzzle = lines
        .map(|line| {
            let (solution, equations) = line.split_once(':').unwrap();
            let solution = solution.parse::<i64>().unwrap();
            let equations = equations
                .trim()
                .split(' ')
                .map(|eq| eq.parse::<i64>().unwrap())
                .collect();

            (solution, equations)
        })
        .collect::<Puzzle>();
    puzzle
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_part_one() {
        let input = std::fs::read_to_string("./resources/day07/example.txt").unwrap();
        let puzzle = parse_puzzle(&input);
        let solution = solve_part_one(&puzzle);
        assert_eq!(solution, "3749".to_string());
    }

    #[test]
    fn test_solve_part_two() {
        let input = std::fs::read_to_string("./resources/day07/example.txt").unwrap();
        let puzzle = parse_puzzle(&input);
        let solution = solve_part_two(&puzzle);
        assert_eq!(solution, "11387".to_string());
    }
}
