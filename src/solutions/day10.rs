use std::collections::HashSet;

type Size = (u32, u32);
type Position = (i32, i32);
type Puzzle = (Size, Vec<u8>);
pub fn solve(input_file_path: &str) -> (String, String) {

    let input = std::fs::read_to_string(input_file_path).unwrap();
    let puzzle = parse_puzzle(&input);
    let (solution_part_one, solution_part_two) = solve_part_one(&puzzle);
    (solution_part_one.to_string(), solution_part_two.to_string())
}

fn solve_part_one(puzzle: &Puzzle) -> (i64, i64) {
    let ((width, height), data) = puzzle;

    let get_pos_value = |(x, y): &Position| -> u8 {
        return data[(y * (*width as i32) + x) as usize];
    };

    let out_of_bound = |(x, y): &Position| -> bool {
        return *x < 0 || *y < 0 || *x >= *width as i32 || *y >= *height as i32;
    };

    let mut sum_trailhead_score = 0;
    let mut sum_unique_paths = 0;

    for y in 0..*height {
        for x in 0..*width {
            let pos = (x as i32, y as i32);
            if get_pos_value(&pos) == '0' as u8 {
                let mut reachable_ends: HashSet<Position> = HashSet::new();
                let unique_paths =
                    explore_hiking_path(pos, &get_pos_value, &out_of_bound, &mut reachable_ends);
                sum_unique_paths = sum_unique_paths + unique_paths;
                sum_trailhead_score += reachable_ends.len();
            }
        }
    }
    (sum_trailhead_score as i64, sum_unique_paths as i64)
}

fn explore_hiking_path(
    start_pos: Position,
    get_pos_value: &impl Fn(&Position) -> u8,
    out_of_bound: &impl Fn(&Position) -> bool,
    reachable_ends: &mut HashSet<Position>,
) -> u32 {
    let directions: Vec<(i32, i32)> = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];

    let current_pos_value = get_pos_value(&start_pos);
    if current_pos_value == '9' as u8 {
        reachable_ends.insert(start_pos);
        return 1;
    }
    let mut unique_paths_sum = 0;
    for direction in directions {
        let next_pos = (start_pos.0 + direction.0, start_pos.1 + direction.1);

        if !out_of_bound(&next_pos) {}
        if !out_of_bound(&next_pos) && get_pos_value(&next_pos) == current_pos_value + 1 {
            let unique_paths =
                explore_hiking_path(next_pos, get_pos_value, out_of_bound, reachable_ends);
            unique_paths_sum += unique_paths
        }
    }
    unique_paths_sum
}

fn parse_puzzle(input: &str) -> Puzzle {
    let width = input.lines().nth(0).unwrap().len() as u32;
    let height = input.lines().count() as u32;
    (
        (width, height),
        input
            .replace('\r', "")
            .replace('\n', "")
            .to_ascii_lowercase()
            .chars()
            .map(|c| c as u8)
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_both_parts() {
        let input = std::fs::read_to_string("./resources/day10/example.txt").unwrap();
        let puzzle = parse_puzzle(&input);
        let (solution_part_one, solution_part_two) = solve_part_one(&puzzle);
        assert_eq!(solution_part_one.to_string(), "36");
        assert_eq!(solution_part_two.to_string(), "81");
    }
}
