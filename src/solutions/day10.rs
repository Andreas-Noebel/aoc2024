use std::collections::HashSet;

type GridDimension = (i16, i16);
type Position = (i16, i16);
type Puzzle = (GridDimension, Vec<u8>);
pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();
    let puzzle = parse_puzzle(&input);
    let (solution_part_one, solution_part_two) = solve_both_parts(&puzzle);
    (solution_part_one.to_string(), solution_part_two.to_string())
}

fn solve_both_parts(puzzle: &Puzzle) -> (u16, u16) {
    let ((width, height), data) = puzzle;

    let lookup_pos_value = |(x, y): &Position| -> u8 {
        return data[(y * (*width) + x) as usize];
    };

    let is_out_of_bounds = |(x, y): &Position| -> bool {
        return *x < 0 || *y < 0 || *x >= *width || *y >= *height;
    };

    let directions: Vec<Position> = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut sum_trailhead_score: u16 = 0;
    let mut sum_unique_paths: u16 = 0;

    for y in 0..*height {
        for x in 0..*width {
            let start_pos = (x, y);
            if lookup_pos_value(&start_pos) == '0' as u8 {
                let mut reachable_ends: HashSet<Position> = HashSet::new();
                let unique_paths = explore_hiking_path(
                    start_pos,
                    &directions,
                    &lookup_pos_value,
                    &is_out_of_bounds,
                    &mut reachable_ends,
                );
                sum_unique_paths = sum_unique_paths + unique_paths;
                sum_trailhead_score += reachable_ends.len() as u16;
            }
        }
    }
    (sum_trailhead_score, sum_unique_paths)
}

fn explore_hiking_path(
    start_pos: Position,
    directions: &Vec<Position>,
    lookup_pos_value: &impl Fn(&Position) -> u8,
    is_out_of_bounds: &impl Fn(&Position) -> bool,
    reachable_ends: &mut HashSet<Position>,
) -> u16 {
    let current_pos_value = lookup_pos_value(&start_pos);
    if current_pos_value == '9' as u8 {
        reachable_ends.insert(start_pos);
        return 1;
    }
    let mut unique_paths_sum = 0;
    for &direction in directions {
        let next_pos = (start_pos.0 + direction.0, start_pos.1 + direction.1);

        if !is_out_of_bounds(&next_pos) && lookup_pos_value(&next_pos) == current_pos_value + 1 {
            let unique_paths = explore_hiking_path(
                next_pos,
                &directions,
                lookup_pos_value,
                is_out_of_bounds,
                reachable_ends,
            );
            unique_paths_sum += unique_paths
        }
    }
    unique_paths_sum
}

fn parse_puzzle(input: &str) -> Puzzle {
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    (
        (width as i16, height as i16),
        input
            .replace('\r', "")
            .replace('\n', "")
            .as_bytes()
            .to_vec(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_both_parts() {
        let input = std::fs::read_to_string("./resources/day10/example.txt").unwrap();
        let puzzle = parse_puzzle(&input);
        let (solution_part_one, solution_part_two) = solve_both_parts(&puzzle);
        assert_eq!(solution_part_one.to_string(), "36");
        assert_eq!(solution_part_two.to_string(), "81");
    }
}
