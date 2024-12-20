use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::iter::{repeat, zip};

pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();
    let puzzle = parse(&input);
    let part_one = solve_part_one(&puzzle);
    let part_two = solve_part_two(&puzzle);
    (part_one, part_two)
}

fn solve_part_one(puzzle: &Maze) -> String {
    puzzle
        .get_shortcuts(2)
        .iter()
        .filter(|(_, c, _)| *c >= 100)
        .count()
        .to_string()
}

fn solve_part_two(puzzle: &Maze) -> String {
    puzzle
        .get_shortcuts(20)
        .iter()
        .filter(|(_, c, _)| *c >= 100)
        .count()
        .to_string()
}

struct Maze {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

type Position = (usize, usize);
type Costs = usize;

impl Maze {
    fn get_start_pos(&self) -> (usize, usize) {
        let start_index = self.data.iter().position(|&x| x == b'S').unwrap_or(0);
        (start_index % self.width, start_index / self.width)
    }

    fn get_end_pos(&self) -> (usize, usize) {
        let end_index = self.data.iter().position(|&x| x == b'E').unwrap_or(0);
        (end_index % self.width, end_index / self.width)
    }

    fn get_costs_to_position(&self, target_position: &Position) -> Result<Vec<Costs>, String> {
        let mut open_set: VecDeque<(usize, usize)> = VecDeque::new();
        let mut closed_set = HashSet::new();
        let mut costs_to_target: Vec<usize> = vec![usize::MAX; self.data.len()];

        open_set.push_back(*target_position);
        costs_to_target[target_position.1 * self.width + target_position.0] = 0;

        while !open_set.is_empty() {
            let current_pos = open_set.pop_front().unwrap();
            let current_cost = costs_to_target[current_pos.1 * self.width + current_pos.0];
            if closed_set.contains(&current_pos) {
                continue;
            }
            closed_set.insert(current_pos);

            let neighbors = vec![
                (current_pos.0 - 1, current_pos.1),
                (current_pos.0 + 1, current_pos.1),
                (current_pos.0, current_pos.1 - 1),
                (current_pos.0, current_pos.1 + 1),
            ];

            neighbors
                .iter()
                .filter(|&&pos| self.is_position_free(pos))
                .filter(|&&pos| !closed_set.contains(&pos))
                .for_each(|&neighbor| {
                    costs_to_target[neighbor.1 * self.width + neighbor.0] = current_cost + 1;
                    open_set.push_back(neighbor);
                });
        }
        Ok(costs_to_target)
    }

    fn get_shortcuts(&self, steps: u32) -> Vec<(Position, Costs, Position)> {
        let costs_to_end = self.get_costs_to_position(&self.get_end_pos()).unwrap();
        let costs_to_start = self.get_costs_to_position(&self.get_start_pos()).unwrap();

        let start_pos = self.get_start_pos();
        let solution_without_shortcuts = costs_to_end[start_pos.1 * self.width + start_pos.0];

        let mut cheats: Vec<(Position, Costs, Position)> =
            Vec::with_capacity((steps * steps * 4) as usize);

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let shortcut_end_pos = self.get_cheat_end_positions((x, y), steps);
                let shortcuts = zip(repeat((x, y)), shortcut_end_pos);

                for (a, b) in shortcuts {
                    if !self.is_position_free(a) || !self.is_position_free(b) {
                        continue;
                    }

                    let cost_a_to_start = costs_to_start[a.1 * self.width + a.0];
                    let cost_b_to_end = costs_to_end[b.1 * self.width + b.0];

                    let cheat_costs = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);

                    let discounted_path_costs =
                        cost_a_to_start + cost_b_to_end + cheat_costs as Costs;
                    if discounted_path_costs >= solution_without_shortcuts {
                        continue;
                    }
                    let discount = solution_without_shortcuts - discounted_path_costs;

                    cheats.push((a, discount, b));
                }
            }
        }
        cheats
    }

    fn get_cheat_end_positions(&self, (sx, sy): Position, steps: u32) -> HashSet<Position> {
        let mut cheat_end_positions: HashSet<Position> = HashSet::new();
        let sx = sx as i32;
        let sy = sy as i32;
        let steps = steps as i32;

        for y in 0..steps + 1 {
            for x in 0..steps + 1 {
                if x + y > steps || (x == 0 && y == 0) {
                    continue;
                }
                let step_end_pos = vec![
                    (sx + x, sy + y),
                    (sx + x, sy - y),
                    (sx - x, sy + y),
                    (sx - x, sy - y),
                ];
                step_end_pos
                    .iter()
                    .filter(|(px, py)| {
                        px >= &0
                            && py >= &0
                            && px < &(self.width as i32)
                            && py < &(self.height as i32)
                    })
                    .filter(|(px, py)| {
                        let p = (*px as usize, *py as usize);
                        self.is_position_free(p)
                    })
                    .map(|(px, py)| (*px as usize, *py as usize))
                    .for_each(|(px, py)| {
                        cheat_end_positions.insert((px, py));
                    });
            }
        }
        cheat_end_positions
    }

    fn is_position_free(&self, (x, y): (usize, usize)) -> bool {
        self.data[y * self.width + x] != b'#'
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Width: {} , Height: {}", self.width, self.height,)
    }
}

fn parse(input: &str) -> Maze {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    let flat_input = input.replace("\n", "").replace("\r", "");

    Maze {
        width,
        height,
        data: flat_input.as_bytes().to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("./resources/day20/example.txt").unwrap();
        let puzzle = parse(&input);

        assert_eq!(puzzle.get_start_pos(), (1, 3));
        assert_eq!(puzzle.get_end_pos(), (5, 7));
        assert_eq!(puzzle.width, 15);
        assert_eq!(puzzle.height, 15);
    }
}
